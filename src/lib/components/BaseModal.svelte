<script lang="ts">
  export let open: boolean = false;
  export let close: Function = () => (open = false);
</script>

{#if open}
  <div
    class="absolute top-0 bottom-0 left-0 right-0 h-screen grid justify-center items-center"
  >
    <!-- svelte-ignore a11y-click-events-have-key-events -->
    <!-- svelte-ignore a11y-no-static-element-interactions -->
    <div
      on:click={() => close()}
      class="absolute top-0 bottom-0 left-0 right-0 h-screen bg-gray-900/90"
      style="z-index: 100;"
    />
    <div
      class="relative flex flex-col gap-y-4 bg-white p-4 rounded-md"
      style="z-index: 1000;"
    >
      <slot />
    </div>
  </div>
{/if}
