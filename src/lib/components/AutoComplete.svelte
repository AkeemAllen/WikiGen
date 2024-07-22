<script lang="ts">
import { Autocomplete, popup } from "@skeletonlabs/skeleton";
import { IconChevronDown } from "@tabler/icons-svelte";

export let id: string = "";
export let popupId: string = "";
export let placeholder: string = "";
export let label: string = "";
export let value: string | number | null = "";
export let showChevron: boolean = true;
export let disabled: boolean = false;
export let options: { label: string; value: string | number | null }[] = [];
export let onSelection: (event: CustomEvent) => void;
export let inputNode: HTMLInputElement = document.createElement("input");
export let onKeydown: (event: KeyboardEvent) => void = () => {};
function setInputNode(node: HTMLElement, input: HTMLElement) {
  inputNode = node as HTMLInputElement;
}

let className: string = "";
export { className as class };
</script>

<div class={className}>
  <label for={id} class="block text-sm font-medium leading-6 text-gray-900">
    {label}
  </label>
  <div class="relative">
    <input
      id={id}
      type="text"
      placeholder={placeholder}
      class="mt-2 block w-full rounded-md border-0 py-1.5 pl-2 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 disabled:bg-gray-100 disabled:text-gray-400 sm:text-sm sm:leading-6"
      bind:value={value}
      disabled={disabled}
      use:popup={{
      event: "focus-click",
      target: popupId,
      placement: "bottom",
    }}
      use:setInputNode={inputNode}
      on:keydown={onKeydown}
    />
    {#if showChevron}
      <div class="absolute right-5 top-[10px] z-10 block">
        <IconChevronDown size={18} class="text-gray-400" />
      </div>
    {/if}
  </div>
  <div
    data-popup={popupId}
    class="card z-10 mt-2 w-60 overflow-y-auto rounded-sm bg-white"
    tabindex="-1"
  >
    <Autocomplete
      bind:input={value}
      options={options}
      limit={5}
      on:selection={onSelection}
      class="w-full rounded-md border bg-white p-2 text-sm"
      emptyState="Creating a new option..."
    />
  </div>
</div>
