<script lang="ts">
  import { base64ToArray } from "$lib/utils";
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
  import { Button } from "$lib/components/ui/button";
  import { onMount } from "svelte";
  import { Label } from "$lib/components/ui/label";
  import { Input } from "$lib/components/ui/input";
  import { Textarea } from "$lib/components/ui/textarea";
  import * as Card from "$lib/components/ui/card";
  import { toast } from "svelte-sonner";

  let homePageImage: string = $state("");
  let generalInfo = $state("");

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
    await writeTextFile(
      `${$selectedWiki.name}/dist/docs/index.md`,
      uploadedInfo,
      { baseDir: BaseDirectory.AppData },
    ).then(() => {
      toast.success("Changes saved!");
    });
  }

  function onImageUpload(e: any) {
    let file = e.target.files[0];
    let reader = new FileReader();
    reader.onloadend = (e) => {
      let base64 = e.target?.result as string;
      if (!base64.includes("data:image/png;base64,")) {
        toast.error("Invalid image format!");
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

<Card.Root class="mx-5 mt-5">
  <Card.Content>
    <Button onclick={saveChanges}>Save Changes</Button>
    <div class="mt-4">
      <Label
        for="home-page-image"
        class="block text-sm font-medium leading-6 text-gray-900"
        >Home Page Logo</Label
      >
      {#if homePageImage !== ""}
        <img src={homePageImage} alt="Home Page" />
      {/if}
      <Input
        id="home-page-image"
        type="file"
        accept="image/png"
        class="mt-2"
        onchange={onImageUpload}
      />
    </div>
    <div class="mt-4">
      <Label
        for="effect"
        class="block text-sm font-medium leading-6 text-gray-900"
        >General Information</Label
      >
      <div class="mt-2">
        <Textarea
          id="effect"
          bind:value={generalInfo}
          spellcheck="false"
          onkeydown={captureTab}
        />
      </div>
    </div>
  </Card.Content>
</Card.Root>
