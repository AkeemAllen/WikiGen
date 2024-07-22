<script lang="ts">
import { IconChevronDown } from "@tabler/icons-svelte";
import type { EventHandler } from "svelte/elements";
type Option = {
  label: string;
  value: any;
};
export let id = "";
export let label = "";
export let value: string | undefined | null | number = "";
export let options: Option[] | string[] = [];
export let onChange: EventHandler<Event, HTMLSelectElement> = () => {};
</script>

<div class="min-w-20">
  <label for={id} class=" block text-sm font-medium leading-6 text-gray-900"
    >{label}</label
  >
  <div class="relative">
    <select
      id={id}
      style={"-webkit-appearance: none;"}
      class=" mt-2 block w-full rounded-md border-0 py-1.5 pl-2 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6"
      bind:value={value}
      on:change={onChange}
    >
      {#each options as option}
        {#if typeof option === "string"}
          <option value={option}>{option}</option>
        {:else}
          <option value={option.value}>{option.label}</option>
        {/if}
      {/each}
    </select>
    <div class="absolute right-5 top-[10px] z-10 block">
      <IconChevronDown size={18} class="text-gray-400" />
    </div>
  </div>
</div>
