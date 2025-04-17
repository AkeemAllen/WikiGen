<script lang="ts">
  import { base64ToArray } from "$lib/utils";
  import { getToastStore } from "@skeletonlabs/skeleton";
  import {
    BaseDirectory,
    exists,
    readFile,
    readTextFile,
    remove,
    writeFile,
    writeTextFile,
  } from "@tauri-apps/plugin-fs";
  import { selectedWiki } from "../../store";
  import Button from "$lib/components/Button.svelte";
  import { onMount } from "svelte";

  let homePageImage: string = "";
  let generalInfo = "";

  const toastStore = getToastStore();

  onMount(async () => {
    homePageImage = await readFile(
      `${$selectedWiki.name}/dist/docs/img/logo.png`,
      { baseDir: BaseDirectory.AppData },
    )
      .then((res) => {
        const blob = new Blob([res], { type: "image/png" });
        return URL.createObjectURL(blob);
      })
      .catch(() => {
        return "";
      });
    generalInfo = await readTextFile(
      `${$selectedWiki.name}/dist/docs/index.md`,
      { baseDir: BaseDirectory.AppData },
    ).then((res) => {
      return res.replace('<img alt="home-page" src="img/logo.png">\n\n', "");
    });
  });

  async function saveHomePageImage() {
    // Write image to file
    if (homePageImage.includes("localhost")) return;
    if (homePageImage === "") {
      const previousImageExists = await exists(
        `${$selectedWiki.name}/dist/docs/img/logo.png`,
        {
          baseDir: BaseDirectory.AppData,
        },
      );

      if (!previousImageExists) {
        return;
      }
      await remove(`${$selectedWiki.name}/dist/docs/img/logo.png`, {
        baseDir: BaseDirectory.AppData,
      });
      return;
    }
    console.log(homePageImage);
    const imageBytes = base64ToArray(
      homePageImage.replace("data:image/png;base64,", ""),
      "image/png",
    );
    await writeFile(
      `${$selectedWiki.name}/dist/docs/img/logo.png`,
      new Uint8Array(imageBytes),
      { baseDir: BaseDirectory.AppData },
    ).catch((e) => {
      console.error(e);
    });
  }

  async function saveChanges() {
    await saveHomePageImage();
    let uploadedInfo =
      '<img alt="home-page" src="img/logo.png">\n\n' + generalInfo;
    if (homePageImage === "") {
      uploadedInfo = generalInfo.replace(
        '<img alt="home-page" src="img/logo.png">\n\n',
        "",
      );
    }
    console.log(uploadedInfo);
    await writeTextFile(
      `${$selectedWiki.name}/dist/docs/index.md`,
      uploadedInfo,
      { baseDir: BaseDirectory.AppData },
    ).then(() => {
      toastStore.trigger({
        message: "Changes saved!",
        background: "variant-filled-success",
      });
    });
  }

  function onImageUpload(e: any) {
    let file = e.target.files[0];
    let reader = new FileReader();
    reader.onloadend = (e) => {
      let base64 = e.target?.result as string;
      if (!base64.includes("data:image/png;base64,")) {
        toastStore.trigger({
          message: "Invalid image format!",
          background: "variant-filled-error",
        });
        return;
      }
      homePageImage = e.target?.result as string;
    };
    reader.readAsDataURL(file);
  }

  function captureTab(e: any) {
    if (e.key === "Tab") {
      e.preventDefault();
      const start = e.target.selectionStart;
      const end = e.target.selectionEnd;
      generalInfo = `${generalInfo.slice(0, start)}\t${generalInfo.slice(end)}`;
      e.target.selectionStart = e.target.selectionEnd = start + 1;
    }
  }
</script>

<Button onClick={saveChanges} title="Save Changes" />
<div class="mt-4">
  <label
    for="home-page-image"
    class="block text-sm font-medium leading-6 text-gray-900"
    >Home Page Logo</label
  >
  {#if homePageImage !== ""}
    <img src={homePageImage} alt="Home Page" />
  {/if}
  <input
    id="home-page-image"
    type="file"
    accept="image/png"
    class="mt-2"
    on:change={onImageUpload}
  />
</div>
<div class="mt-4">
  <label for="effect" class="block text-sm font-medium leading-6 text-gray-900"
    >General Information</label
  >
  <div class="mt-2">
    <textarea
      id="effect"
      bind:value={generalInfo}
      spellcheck="false"
      on:keydown={captureTab}
      class="block h-48 w-[50rem] rounded-md border-0 py-1.5 pl-2 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 disabled:bg-gray-100 disabled:text-gray-400 sm:text-sm sm:leading-6"
    />
  </div>
</div>
