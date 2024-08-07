<script lang="ts">
  import { IconTrash } from "@tabler/icons-svelte";
  import { type TrainerPokemon } from "../../store/gameRoutes";
  import { BaseDirectory, readBinaryFile } from "@tauri-apps/api/fs";
  import { selectedWiki } from "../../store";
  import capitalizeWords from "$lib/utils/capitalizeWords";

  export let pokemon: TrainerPokemon = {} as TrainerPokemon;
  export let trainerName: string;

  export let deletePokemon = (id: string, name: string) => {};

  async function getSpriteImage(pokemonName: string): Promise<string> {
    return await readBinaryFile(
      `${$selectedWiki.name}/dist/docs/img/pokemon/${pokemonName}.png`,
      { dir: BaseDirectory.AppData },
    )
      .then((res) => {
        const blob = new Blob([res], { type: "image/png" });
        return URL.createObjectURL(blob);
      })
      .catch((err) => {
        console.log(err);
        if (err.includes("No such file or directory")) {
          return "Image Not Found";
        }
        return "Error loading image";
      });
  }
</script>

{#await getSpriteImage(pokemon.name) then spriteUrl}
  <img src={spriteUrl} alt={pokemon.name} class="m-0 justify-self-center" />
{/await}
<div class="w-full rounded-md border-2">
  <p class="text-center">
    {capitalizeWords(pokemon.name)}
  </p>
  <p class="text-center">
    {pokemon.level}
  </p>
</div>
<button
  class="invisible absolute right-2 top-2 z-10 rounded-md bg-red-200 p-1 hover:scale-110 group-hover:visible"
  on:click={(e) => {
    e.stopPropagation();
    deletePokemon(pokemon.unique_id, trainerName);
  }}
>
  <IconTrash size={16} color="white" />
</button>
