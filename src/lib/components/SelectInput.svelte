<script lang="ts">
  import IconChevronDown from "@tabler/icons-svelte/icons/chevron-down";
  import type { EventHandler } from "svelte/elements";
  type Option = {
    label: string;
    value: any;
  };
  interface Props {
    id?: string;
    label?: string;
    value?: string | undefined | null | boolean | number;
    options?: Option[];
    onChange?: EventHandler<Event, HTMLSelectElement>;
    topMargin?: boolean;
    class?: string;
    [key: string]: any;
  }

  let {
    id = "",
    label = "",
    value = $bindable(""),
    options = [],
    onChange = () => {},
    topMargin = false,
    class: className = "",
    ...rest
  }: Props = $props();
</script>

<div class="min-w-20 {className}">
  <label for={id} class="block text-sm font-medium leading-6 text-gray-900"
    >{label}</label
  >
  <div class="relative">
    <select
      {id}
      style={"-webkit-appearance: none;"}
      class="block w-full rounded-md border-0 pl-2 py-1.5 {topMargin
        ? 'm-2'
        : ''} text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6 disabled:bg-gray-100 disabled:text-gray-400"
      bind:value
      onchange={onChange}
      {...rest}
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
