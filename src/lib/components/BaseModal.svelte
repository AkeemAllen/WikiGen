<script lang="ts">
export let open: boolean = false;
export let close: Function = () => (open = false);
let className: string = "";
export { className as class };
</script>

{#if open}
  <div
    class="absolute bottom-0 left-0 right-0 top-0 grid h-screen items-center justify-center"
  >
    <!-- svelte-ignore a11y-click-events-have-key-events -->
    <!-- svelte-ignore a11y-no-static-element-interactions -->
    <div
      on:click={() => close()}
      class="absolute bottom-0 left-0 right-0 top-0 h-screen bg-gray-900/90"
      style="z-index: 100;"
    />
    <div
      class="{className} relative flex flex-col gap-y-4 rounded-md bg-white p-4"
      style="z-index: 1000;"
    >
      <slot />
    </div>
  </div>
{/if}
