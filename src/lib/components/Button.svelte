<script lang="ts">
  export let onClick = () => {};
  export let title: string = "";
  export let disabled: boolean = false;
</script>

<button
  on:click={onClick}
  {disabled}
  class=" rounded-md bg-indigo-600 w-40 px-3 py-2 mt-2 text-sm font-semibold text-white
        shadow-sm hover:bg-indigo-500 focus-visible:outline
        focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600
        disabled:bg-indigo-400"
>
  {title}</button
>
