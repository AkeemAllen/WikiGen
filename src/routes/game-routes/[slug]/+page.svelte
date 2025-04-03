<script lang="ts">
  import TrainerEncounters from "$lib/components/game-route-components/TrainerEncounters.svelte";
  import WildEncounters from "$lib/components/game-route-components/WildEncounters.svelte";
  import SelectInput from "$lib/components/SelectInput.svelte";
  import Button from "$lib/components/Button.svelte";
  import { getToastStore, Tab, TabGroup } from "@skeletonlabs/skeleton";
  import IconTrash from "@tabler/icons-svelte/icons/trash";
  import IconArrowLeft from "@tabler/icons-svelte/icons/arrow-left";
  import { routes } from "../../../store/gameRoutes";
  import { selectedWiki } from "../../../store";
  import {
    BaseDirectory,
    createDir,
    exists,
    readBinaryFile,
    removeFile,
    writeBinaryFile,
  } from "@tauri-apps/api/fs";
  import { onMount } from "svelte";
  import { base64ToArray } from "$lib/utils";
  import { getToastSettings, ToastType } from "$lib/utils/toasts";
  import { generateRoutePages, updateRoutes } from "$lib/utils/generators";

  const toastStore = getToastStore();
  export let data;
  let tabSet: number = 0;
  let newRouteImage: string = "";

  onMount(async () => {
    newRouteImage = await readBinaryFile(
      `${$selectedWiki.name}/dist/docs/img/routes/${data.title}.png`,
      { dir: BaseDirectory.AppData },
    )
      .then((res) => {
        const blob = new Blob([res], { type: "image/png" });
        return URL.createObjectURL(blob);
      })
      .catch(() => {
        return "";
      });
  });

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
      newRouteImage = e.target?.result as string;
    };
    reader.readAsDataURL(file);
  }

  async function saveRouteImage() {
    const routeImagesDirectoryExists = await exists(
      `${$selectedWiki.name}/dist/docs/img/routes`,
      {
        dir: BaseDirectory.AppData,
      },
    );

    if (!routeImagesDirectoryExists) {
      await createDir(`${$selectedWiki.name}/dist/docs/img/routes`, {
        dir: BaseDirectory.AppData,
        recursive: true,
      });
    }

    if (newRouteImage === "") {
      const previousImageExists = await exists(
        `${$selectedWiki.name}/dist/docs/img/routes/${data.title}.png`,
        {
          dir: BaseDirectory.AppData,
        },
      );

      if (!previousImageExists) {
        return;
      }
      await removeFile(
        `${$selectedWiki.name}/dist/docs/img/routes/${data.title}.png`,
        {
          dir: BaseDirectory.AppData,
        },
      );
      return;
    }

    const imageBytes = base64ToArray(
      newRouteImage.replace("data:image/png;base64,", ""),
      "image/png",
    );

    await writeBinaryFile(
      `${$selectedWiki.name}/dist/docs/img/routes/${data.title}.png`,
      imageBytes,
      { dir: BaseDirectory.AppData },
    )
      .then(() => {
        toastStore.trigger(
          getToastSettings(ToastType.SUCCESS, "Route Image Saved"),
        );
      })
      .catch((e) => {
        toastStore.trigger(
          getToastSettings(ToastType.ERROR, `Error saving route image: ${e}`),
        );
      });
  }

  async function saveChanges() {
    await saveRouteImage().catch((e) => {
      toastStore.trigger(
        getToastSettings(ToastType.ERROR, `Error saving route image: ${e}`),
      );
    });
    await updateRoutes($routes, $selectedWiki.name).then(() => {
      generateRoutePages([data.title], $selectedWiki.name)
        .then((res) => {
          toastStore.trigger(
            getToastSettings(ToastType.SUCCESS, res as string),
          );
        })
        .catch((e) => {
          toastStore.trigger(getToastSettings(ToastType.ERROR, e));
        });
    });
  }
</script>

<strong class="text-l flex flex-row items-center gap-5">
  <a href="/game-routes" class="hover:cursor-pointer">
    <IconArrowLeft size={20} />
  </a>
  {data.title}</strong
>
<TabGroup class="mt-4">
  <Tab bind:group={tabSet} name="wild-encounters" value={0} class="text-sm"
    >Wild Encounters</Tab
  >
  <Tab bind:group={tabSet} name="trainer-encounters" value={1} class="text-sm"
    >Trainer Encounters</Tab
  >
  <Tab bind:group={tabSet} name="properties" value={2} class="text-sm"
    >Properties</Tab
  >
  <svelte:fragment slot="panel">
    {#if tabSet === 0}
      <WildEncounters routeName={data.title} />
    {/if}
    {#if tabSet === 1}
      <TrainerEncounters routeName={data.title} />
    {/if}
    {#if tabSet === 2}
      <Button title="Save Changes" onClick={saveChanges} class="w-32" />
      <div class="mb-4 mt-4">
        <label
          for="sprite-image"
          class="block text-sm font-medium leading-6 text-gray-900"
          >Route Image</label
        >
        {#if newRouteImage !== ""}
          <img src={newRouteImage} alt="Route" />
        {/if}
        {#if newRouteImage !== ""}
          <button
            class="flex flex-row items-center gap-2 text-white bg-[#111827] rounded-2xl text-[14px] pt-[4px] pb-[5px] pl-3 pr-3"
            on:click={() => {
              newRouteImage = "";
            }}
          >
            <IconTrash size={14} />
            Clear Image</button
          >
        {/if}
        <input
          id="sprite-image"
          type="file"
          accept="image/png"
          class="mt-2"
          on:change={onImageUpload}
        />
      </div>
      <div class="w-36">
        <SelectInput
          label="Render Route Page"
          bind:value={$routes.routes[data.title].render}
          options={[
            {
              value: true,
              label: "True",
            },
            {
              value: false,
              label: "False",
            },
          ]}
        />
      </div>
    {/if}
  </svelte:fragment>
</TabGroup>
