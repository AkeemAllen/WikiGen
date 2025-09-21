<script lang="ts">
  import TrainerEncounters from "$lib/components/game-route-components/TrainerEncounters.svelte";
  import WildEncounters from "$lib/components/game-route-components/WildEncounters.svelte";
  import { Button } from "$lib/components/ui/button/index";
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
  import { generateRoutePages, updateRoutes } from "$lib/utils/generators";
  import * as Tabs from "$lib/components/ui/tabs/index.js";
  import * as Select from "$lib/components/ui/select/index.js";
  import capitalizeWords from "$lib/utils/capitalizeWords";
  import { Input } from "$lib/components/ui/input";
  import { Label } from "$lib/components/ui/label";
  import { toast } from "svelte-sonner";

  let { data } = $props();
  let newRouteImage: string = $state("");
  let renderRoute: string = $derived.by(() => {
    return $routes.routes[data.title].render.toString();
  });

  onMount(async () => {
    newRouteImage = await readFile(
      `${$selectedWiki.name}/dist/docs/img/routes/${data.title}.png`,
      { baseDir: BaseDirectory.AppData },
    )
      .then((res: any) => {
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
        toast.error("Invalid image format!");
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
        toast.success("Route Image Saved");
      })
      .catch((e) => {
        toast.error(`Error saving route image: ${e}`);
      });
  }

  async function saveChanges() {
    await saveRouteImage().catch((e) => {
      toast.error(`Error saving route image: ${e}`);
    });
    $routes.routes[data.title].render = renderRoute === "true";
    await updateRoutes($routes, $selectedWiki.name).then(() => {
      generateRoutePages([data.title], $selectedWiki.name)
        .then((res) => {
          toast.success(res as string);
        })
        .catch((e) => {
          toast.error(e);
        });
    });
  }

  async function deleteVariant(name: string) {
    let updatedVariants = $routes.routes[data.title].variants.filter(
      (variant) => variant !== name,
    );

    let updatedPokemon = $routes.routes[data.title].wild_encounters.filter(
      (encounter) => encounter.route_variant !== name,
    );

    if (updatedVariants.length !== $routes.routes[data.title].variants.length) {
      $routes.routes[data.title].variants = updatedVariants;
      $routes.routes[data.title].wild_encounters = updatedPokemon;
      await updateRoutes($routes, $selectedWiki.name)
        .then(() => {
          toast.success("Variant deleted successfully");
        })
        .catch((e) => {
          toast.error(`Error deleting variant: ${e}`);
        });
    }
  }
</script>

<strong class="bg-white text-l flex flex-row items-center gap-5 px-5 pt-5">
  <a href="/game-routes" class="hover:cursor-pointer">
    <IconArrowLeft size={20} />
  </a>
  {data.title}</strong
>
<Tabs.Root value="wild-encounters" class="w-full mb-10">
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
    <Button title="Save Changes" onclick={saveChanges} class="w-[15rem]"
      >Save Changes</Button
    >
    <div class="mb-4 mt-4">
      <Label
        for="route-image"
        class="block text-sm font-medium leading-6 text-gray-900"
        >Route Image</Label
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
      <Input
        id="route-image"
        type="file"
        accept="image/png"
        class="mt-2 w-[15rem]"
        onchange={onImageUpload}
      />
    </div>
    <div>
      <Label
        for="render-route"
        class="block text-sm font-medium leading-6 text-gray-900"
        >Route Route Page</Label
      >
      <Select.Root type="single" bind:value={renderRoute}>
        <Select.Trigger id="render-route" class="w-[10rem]">
          {capitalizeWords(renderRoute) || "Select an option"}
        </Select.Trigger>
        <Select.Content>
          <Select.Group>
            <Select.Label>Options</Select.Label>
            <Select.Item value="true">True</Select.Item>
            <Select.Item value="false">False</Select.Item>
          </Select.Group>
        </Select.Content>
      </Select.Root>
    </div>
    {#if $routes.routes[data.title].variants.filter((variant) => variant !== "default").length !== 0}
      <Label
        for="delete-route-variants"
        class="block text-sm font-medium leading-6 text-gray-900 my-4"
        >Delete Route Variants
        <p class="text-red-500">
          WARNING: Delete route variants will delete all pokemon under the
          variant. Perform this action with caution.
        </p></Label
      >
    {/if}
    <div class="flex flex-row my-5 gap-5" id="delete-route-variants">
      {#each $routes.routes[data.title].variants.filter((variant) => variant !== "default") as variant}
        <div class="rounded-md shadow-sm p-1 text-center w-32">
          {capitalizeWords(variant)}
          <button
            class=" cursor-pointer rounded-md bg-red-300 p-1 hover:scale-110"
            onclick={(e) => {
              e.stopPropagation();
              deleteVariant(variant);
            }}
          >
            <IconTrash size={16} color="white" />
          </button>
        </div>
      {/each}
    </div>
  </Tabs.Content>
</Tabs.Root>
