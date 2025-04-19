<script lang="ts">
  import type {
    ChangeEventHandler,
    KeyboardEventHandler,
  } from "svelte/elements";

  interface Props {
    id?: string;
    label?: string;
    placeholder?: string;
    value?: string | null;
    disabled?: boolean;
    inputHandler?: any;
    changeHandler?: ChangeEventHandler<HTMLInputElement>;
    onKeyDownHandler?: KeyboardEventHandler<HTMLInputElement>;
    class?: string;
    [key: string]: any;
  }

  let {
    id = "",
    label = "",
    placeholder = "",
    value = $bindable(),
    disabled = false,
    inputHandler = (e: any) => {},
    changeHandler = (e) => {},
    onKeyDownHandler = () => {},
    class: className = "",
    ...rest
  }: Props = $props();
</script>

<div class={className}>
  {#if label !== ""}
    <label for={id} class="block text-sm font-medium leading-6 text-gray-900"
      >{label}</label
    >
  {/if}
  <div class="mt-2">
    <input
      {id}
      {disabled}
      type="text"
      {placeholder}
      oninput={inputHandler}
      onchange={changeHandler}
      onkeydown={onKeyDownHandler}
      class="block w-full rounded-md border-0 py-1.5 pl-2 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 disabled:bg-gray-100 disabled:text-gray-400 sm:text-sm sm:leading-6"
      bind:value
      {...rest}
    />
  </div>
</div>
