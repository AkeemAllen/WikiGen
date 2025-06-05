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
    create,
    exists,
    readFile,
    remove,
    writeFile,
  } from "@tauri-apps/plugin-fs";
  import { onMount } from "svelte";
  import { base64ToArray } from "$lib/utils";
  import { getToastSettings, ToastType } from "$lib/utils/toasts";
  import { generateRoutePages, updateRoutes } from "$lib/utils/generators";
  import * as Tabs from "$lib/components/ui/tabs/index.js";
  import capitalizeWords from "$lib/utils/capitalizeWords";

  const toastStore = getToastStore();
  let { data } = $props();
  let tabSet: number = $state(0);
  let newRouteImage: string = $state("");

  onMount(async () => {
    newRouteImage = await readFile(
      `${$selectedWiki.name}/dist/docs/img/routes/${data.title}.png`,
      { baseDir: BaseDirectory.AppData },
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
        baseDir: BaseDirectory.AppData,
      },
    );

    if (!routeImagesDirectoryExists) {
      await create(`${$selectedWiki.name}/dist/docs/img/routes`, {
        baseDir: BaseDirectory.AppData,
      });
    }

    if (newRouteImage === "") {
      const previousImageExists = await exists(
        `${$selectedWiki.name}/dist/docs/img/routes/${data.title}.png`,
        {
          baseDir: BaseDirectory.AppData,
        },
      );

      if (!previousImageExists) {
        return;
      }
      await remove(
        `${$selectedWiki.name}/dist/docs/img/routes/${data.title}.png`,
        {
          baseDir: BaseDirectory.AppData,
        },
      );
      return;
    }

    const imageBytes = base64ToArray(
      newRouteImage.replace("data:image/png;base64,", ""),
      "image/png",
    );

    await writeFile(
      `${$selectedWiki.name}/dist/docs/img/routes/${data.title}.png`,
      new Uint8Array(imageBytes),
      { baseDir: BaseDirectory.AppData },
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

<strong class="bg-white text-l flex flex-row items-center gap-5 px-5 pt-5">
  <a href="/game-routes" class="hover:cursor-pointer">
    <IconArrowLeft size={20} />
  </a>
  {data.title}</strong
>
<Tabs.Root value="wild-encounters" class="w-full">
  <div class="w-full bg-white border-b">
    <Tabs.List class="w-[30rem] rounded-sm ml-5 my-3">
      {#each ["wild-encounters", "trainer-encounters", "properties"] as tab}
        <Tabs.Trigger value={tab} class="rounded-sm  cursor-pointer"
          >{capitalizeWords(tab)}</Tabs.Trigger
        >
      {/each}
    </Tabs.List>
  </div>
  <Tabs.Content value="wild-encounters" class="mx-5">
    <WildEncounters routeName={data.title} />
  </Tabs.Content>
  <Tabs.Content value="trainer-encounters" class="mx-5">
    <TrainerEncounters routeName={data.title} />
  </Tabs.Content>
  <Tabs.Content value="properties" class="mx-5">
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
          onclick={() => {
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
        onchange={onImageUpload}
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
  </Tabs.Content>
</Tabs.Root>
