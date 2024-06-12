<script lang="ts">
export let id = "";
export let label = "";
export let placeholder = "";
export let min = 0;
export let max = 100;
export let value: number | null = 0;
export let disabled = false;
let className: string = "";
export { className as class };
</script>

<div class={className}>
  <label for={id} class="block text-sm font-medium leading-6 text-gray-900"
    >{label}</label
  >
  <div class="mt-2">
    <input
      id={id}
      disabled={disabled}
      type="number"
      min={min}
      max={max}
      placeholder={placeholder}
      class="block w-full rounded-md border-0 py-1.5 pl-2 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 disabled:bg-gray-100 disabled:text-gray-400 sm:text-sm sm:leading-6"
      bind:value={value}
    />
  </div>
</div>
