<script lang="ts">
import { Autocomplete, popup } from "@skeletonlabs/skeleton";

export let id: string = "";
export let popupId: string = "";
export let placeholder: string = "";
export let label: string = "";
export let value: string = "";
export let options: { label: string; value: string | number }[] = [];
export let onSelection: (event: CustomEvent) => void;
</script>

<div class="w-60">
  <label for={id} class="block text-sm font-medium leading-6 text-gray-900">
    {label}
  </label>
  <input
    id={id}
    type="text"
    placeholder={placeholder}
    class="mt-2 block w-full rounded-md border-0 py-1.5 pl-2 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 disabled:bg-gray-100 disabled:text-gray-400 sm:text-sm sm:leading-6"
    bind:value={value}
    use:popup={{
      event: "focus-click",
      target: popupId,
      placement: "bottom",
    }}
  />
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
    />
  </div>
</div>
