<script lang="ts">
import type { KeyboardEventHandler } from "svelte/elements";

export let id = "";
export let label = "";
export let placeholder = "";
export let value: string | null = "";
export let disabled = false;
export let inputHandler = () => {};
export let onKeyDownHandler: KeyboardEventHandler<HTMLInputElement> = () => {};
let className = "";
export { className as class };
</script>

<div class={className}>
  {#if label !== ""}
    <label for={id} class="block text-sm font-medium leading-6 text-gray-900"
      >{label}</label
    >
  {/if}
  <div class="mt-2">
    <input
      id={id}
      disabled={disabled}
      type="text"
      placeholder={placeholder}
      on:input={inputHandler}
      on:keydown={onKeyDownHandler}
      class="block w-full rounded-md border-0 py-1.5 pl-2 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 disabled:bg-gray-100 disabled:text-gray-400 sm:text-sm sm:leading-6"
      bind:value={value}
      {...$$restProps}
    />
  </div>
</div>
