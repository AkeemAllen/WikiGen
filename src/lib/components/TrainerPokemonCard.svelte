<script lang="ts">
import { IconTrash } from "@tabler/icons-svelte";
import { type TrainerPokemon } from "../../store/gameRoutes";
import { pokemon as storePokemon } from "../../store/pokemon";
import _ from "lodash";

export let pokemon: TrainerPokemon = {} as TrainerPokemon;
export let trainerName: string;

export let deletePokemon = (id: string, name: string) => {};
</script>

<img
  src={$storePokemon.pokemon[pokemon.id].sprite}
  alt={pokemon.name}
  class="m-0 justify-self-center"
/>
<div class="w-full rounded-md border-2">
  <p class="text-center">
    {_.capitalize(pokemon.name)}
  </p>
  <p class="text-center">
    {pokemon.level}
  </p>
</div>
<button
  class="invisible absolute right-2 top-2 z-10 rounded-md bg-red-200 p-1 hover:scale-110 group-hover:visible"
  on:click={(e) => {e.stopPropagation();deletePokemon(pokemon.unique_id, trainerName)}}
>
  <IconTrash size={16} color="white" />
</button>
