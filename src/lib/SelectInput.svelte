<script lang="ts">
  import { IconChevronDown } from "@tabler/icons-svelte";

  type Option = {
    label: string;
    value: any;
  };
  export let id = "";
  export let label = "";
  export let value: string | undefined | null = "";
  export let options: Option[] = [];
</script>

<div>
  <label for={id} class=" block text-sm font-medium leading-6 text-gray-900"
    >{label}</label
  >
  <div class="relative">
    <select
      {id}
      style={"-webkit-appearance: none;"}
      class=" block w-full rounded-md border-0 pl-2 py-1.5 mt-2 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6"
      bind:value
    >
      {#each options as option}
        <option value={option.value}>{option.label}</option>
      {/each}
    </select>
    <div class="absolute block z-10 right-5 top-[10px]">
      <IconChevronDown size={18} class="text-gray-400" />
    </div>
  </div>
</div>
