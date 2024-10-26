import Database from "tauri-plugin-sql-api";
import type { Wiki } from "../../store";
import { db } from "../../store/db";
import { get } from "svelte/store";
import { pokemonList, type SearchPokemon } from "../../store/pokemon";
import { itemsList, type SearchItem } from "../../store/items";
import { abilitiesList, type SearchAbility } from "../../store/abilities";
import { naturesList, type SearchNature } from "../../store/natures";
import { moveList, type SearchMove } from "../../store/moves";
import { BaseDirectory, readTextFile } from "@tauri-apps/api/fs";
import { routes } from "../../store/gameRoutes";
import { sortRoutesByPosition } from "$lib/utils";
import { types } from "../../store/types";
import { ToastType, getToastSettings } from "./toasts";

async function loadRoutes(wikiName: string) {
  const routesFromFile = await readTextFile(`${wikiName}/data/routes.json`, {
    dir: BaseDirectory.AppData,
  });

  routes.set(sortRoutesByPosition(JSON.parse(routesFromFile as string)));
}

async function loadTypes(wikiName: string) {
  const typesFromFile: any = await readTextFile(`${wikiName}/data/types.json`, {
    dir: BaseDirectory.AppData,
  });
  types.set(JSON.parse(typesFromFile)["types"]);
}

export async function loadWikiData(wiki: Wiki, toastStore: any) {
  await Database.load(`sqlite:${wiki.name}/${wiki.name}.db`)
    .then((database) => {
      db.set(database);
      // Load Pokemon
      get(db)
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
      get(db)
        .select("SELECT id, name FROM items")
        .then((items: any) => {
          itemsList.set(items.map((item: SearchItem) => [item.id, item.name]));
        });

      // Load Abilities
      get(db)
        .select("SELECT id, name FROM abilities")
        .then((abilities: any) => {
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
      get(db)
        .select("SELECT id, name FROM natures")
        .then((natures: any) => {
          naturesList.set(
            natures.map((nature: SearchNature) => [nature.id, nature.name]),
          );
        });

      // Load Moves
      get(db)
        .select("SELECT id, name FROM moves")
        .then((moves: any) => {
          moveList.set(moves.map((move: SearchMove) => [move.id, move.name]));
        });

      // Load Types
      loadTypes(wiki.name).catch((err) => {
        toastStore.trigger(
          getToastSettings(ToastType.ERROR, `Error loading types: ${err}`),
        );
      });

      // Load Routes
      loadRoutes(wiki.name).catch((err) => {
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
