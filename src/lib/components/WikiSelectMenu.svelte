<script lang="ts">
  import {
    BaseDirectory,
    readTextFile,
    removeDir,
    writeTextFile,
  } from "@tauri-apps/api/fs";
  import { selectedWiki, wikis, type Wiki } from "../../store";
  import { routes } from "../../store/gameRoutes";
  import { moveList, type SearchMove } from "../../store/moves";
  import { pokemonList, type SearchPokemon } from "../../store/pokemon";
  import { sortRoutesByPosition } from "$lib/utils";
  import { abilitiesList, type SearchAbility } from "../../store/abilities";
  import { naturesList, type SearchNature } from "../../store/natures";
  import { itemsList, type SearchItem } from "../../store/items";
  import BaseModal from "./BaseModal.svelte";
  import MultiSelect from "svelte-multiselect";
  import Button from "./Button.svelte";
  import { getToastStore } from "@skeletonlabs/skeleton";
  import { invoke, os } from "@tauri-apps/api";
  import { goto } from "$app/navigation";
  import Database from "tauri-plugin-sql-api";
  import { db } from "../../store/db";
  import { types } from "../../store/types";
  import HotKeysModal from "./modals/HotKeysModal.svelte";
  import { getToastSettings, ToastType } from "$lib/utils/toasts";
  import { onMount } from "svelte";

  const toastStore = getToastStore();

  let deleteWikiModalOpen: boolean = false;
  let wikisToDelete: string[] = [];
  let directoriesRemoved: boolean = false;
  let wikiJsonUpdated: boolean = false;

  let hotKeysModalOpen: boolean = false;

  $: wikiListOptions = Object.keys($wikis).filter(
    (wiki) => wiki !== $selectedWiki.name,
  );

  async function updatePageNames() {
    await invoke("update_pokemon_pages_with_stripped_name_with_handle", {
      wikiName: $selectedWiki.name,
    })
      .then((res) => {
        toastStore.trigger(
          getToastSettings(
            ToastType.SUCCESS,
            "Pokemon Page Names Updated Successfully",
          ),
        );
      })
      .catch((err) => {
        toastStore.trigger(getToastSettings(ToastType.ERROR, err as string));
      });
  }

  async function loadRoutes() {
    const routesFromFile = await readTextFile(
      `${$selectedWiki.name}/data/routes.json`,
      { dir: BaseDirectory.AppData },
    );

    routes.set(sortRoutesByPosition(JSON.parse(routesFromFile as string)));
  }

  async function loadTypes() {
    const typesFromFile: any = await readTextFile(
      `${$selectedWiki.name}/data/types.json`,
      { dir: BaseDirectory.AppData },
    );
    types.set(JSON.parse(typesFromFile)["types"]);
  }

  async function deleteWikis() {
    for (const wiki of wikisToDelete) {
      await removeDir(wiki, {
        dir: BaseDirectory.AppData,
        recursive: true,
      }).then(() => {
        directoriesRemoved = true;
      });
      $wikis = Object.fromEntries(
        Object.entries($wikis).filter(([key, _]) => key !== wiki),
      );
    }
    await writeTextFile("wikis.json", JSON.stringify($wikis), {
      dir: BaseDirectory.AppData,
    }).then(() => {
      wikiJsonUpdated = true;
    });
    if (directoriesRemoved && wikiJsonUpdated) {
      toastStore.trigger({
        message: "Wikis Deleted Successfully",
        background: "variant-filled-success",
      });
      directoriesRemoved = false;
      wikiJsonUpdated = false;
    }
    deleteWikiModalOpen = false;
    wikisToDelete = [];
  }

  async function backupWiki() {
    await invoke("backup_wiki", {
      wikiName: $selectedWiki.name,
    }).then(() => {
      toastStore.trigger({
        message: "Wiki Backed Up Successfully",
        background: "variant-filled-success",
      });
    });
  }

  async function loadWikiData() {
    await Database.load(`sqlite:${$selectedWiki.name}/${$selectedWiki.name}.db`)
      .then((database) => {
        db.set(database);
        // Load Pokemon
        $db
          .select(
            "SELECT id, dex_number, name, types FROM pokemon ORDER BY dex_number",
          )
          .then((pokemon: any) => {
            pokemonList.set(
              pokemon.map((p: SearchPokemon) => [
                p.id,
                p.dex_number,
                p.name,
                p.types,
              ]),
            );
          });

        // Load Items
        $db.select("SELECT id, name FROM items").then((items: any) => {
          itemsList.set(items.map((item: SearchItem) => [item.id, item.name]));
        });

        // Load Abilities
        $db.select("SELECT id, name FROM abilities").then((abilities: any) => {
          abilitiesList.set(
            abilities.map((ability: SearchAbility) => [
              ability.id,
              ability.name,
            ]),
          );
          // Add an empty ability for the search
          abilitiesList.update((abilities) => {
            abilities.unshift([0, "None"]);
            return abilities;
          });
        });

        // Load Natures
        $db.select("SELECT id, name FROM natures").then((natures: any) => {
          naturesList.set(
            natures.map((nature: SearchNature) => [nature.id, nature.name]),
          );
        });

        // Load Moves
        $db.select("SELECT id, name FROM moves").then((moves: any) => {
          moveList.set(moves.map((move: SearchMove) => [move.id, move.name]));
        });

        // Load Types
        loadTypes().catch((err) => {
          toastStore.trigger(
            getToastSettings(ToastType.ERROR, `Error loading types: ${err}`),
          );
        });

        // Load Routes
        loadRoutes().catch((err) => {
          toastStore.trigger(
            getToastSettings(ToastType.ERROR, `Error loading routes: ${err}`),
          );
        });
      })
      .catch((err) => {
        toastStore.trigger(
          getToastSettings(
            ToastType.ERROR,
            `Error loading values from database: ${err}`,
          ),
        );
      });
  }
</script>

<BaseModal bind:open={deleteWikiModalOpen}>
  <div>
    <label for="wikis" class="block text-sm font-medium leading-6 text-gray-900"
      >Wikis To Delete</label
    >
    <MultiSelect
      id="wikis"
      bind:selected={wikisToDelete}
      options={wikiListOptions}
      style="height: 36px; border-color: rgb(209 213 219); border-radius: 0.375rem; box-shadow: 0 1px 2px 0 rgb(0 0 0 / 0.05); font-size: 0.875rem;"
    />
  </div>
  <Button onClick={() => deleteWikis()} title="Delete Selected Wikis" />
</BaseModal>

<BaseModal bind:open={hotKeysModalOpen} class="w-[40rem]">
  <HotKeysModal />
</BaseModal>

<div
  class="card z-10 max-w-52 grid-cols-1 p-4 shadow-xl"
  data-popup="wikiSelectPopup"
>
  {#if Object.keys($wikis).length !== 0}
    <p class="mb-1 text-xs text-slate-400">Wikis</p>
    {#each Object.entries($wikis) as [_, value]}
      {#if value.name !== $selectedWiki.name}
        <button
          on:click={() => {
            $selectedWiki = value;
            // Check if file migration is needed before loading wiki
            loadWikiData();
            goto("/");
          }}
          class="w-full rounded-md p-2 text-left text-sm hover:bg-slate-300"
          >{value.site_name}</button
        >
      {/if}
    {/each}
  {/if}
  <p class="mb-1 text-xs text-slate-400">Actions</p>
  <a href="/create-wiki">
    <button class="w-full rounded-md p-2 text-left text-sm hover:bg-slate-300"
      >Create New Wiki</button
    >
  </a>
  {#if $selectedWiki.name}
    <button
      class="w-full rounded-md p-2 text-left text-sm hover:bg-slate-300"
      on:click={backupWiki}>Backup Wiki</button
    >
    <button
      class="w-full rounded-md p-2 text-left text-sm hover:bg-slate-300"
      on:click={updatePageNames}>Reformat Pokemon Page Names</button
    >
  {/if}
  <button
    class="w-full rounded-md p-2 text-left text-sm hover:bg-slate-300"
    on:click={() => (hotKeysModalOpen = true)}>View Hotkeys</button
  >
  <button
    class="w-full rounded-md p-2 text-left text-sm text-red-500 hover:bg-slate-300"
    on:click={() => (deleteWikiModalOpen = true)}>Delete A Wiki</button
  >
</div>
