<script lang="ts">
  export let id = "";
  export let label = "";
  export let placeholder = "";
  export let value: string | null = "";
  export let disabled = false;
</script>

<div>
  <label for={id} class="block text-sm font-medium leading-6 text-gray-900"
    >{label}</label
  >
  <div class="mt-2">
    <input
      {id}
      {disabled}
      type="text"
      {placeholder}
      class="block w-full pl-2 rounded-md border-0 py-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6 disabled:bg-gray-100 disabled:text-gray-400"
      bind:value
    />
  </div>
</div>
