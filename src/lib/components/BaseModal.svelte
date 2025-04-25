<script lang="ts">
  interface Props {
    open?: boolean;
    close?: Function;
    class?: string;
    children?: import('svelte').Snippet;
  }

  let {
    open = $bindable(false),
    close = () => (open = false),
    class: className = "",
    children
  }: Props = $props();

</script>

{#if open}
  <div
    class="absolute bottom-0 left-0 right-0 top-0 grid h-screen items-center justify-center"
  >
    <!-- svelte-ignore a11y_click_events_have_key_events -->
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <div
      onclick={() => close()}
      class="absolute bottom-0 left-0 right-0 top-0 h-screen bg-gray-900/90"
      style="z-index: 100;"
></div>
    <div
      class="{className} relative flex flex-col gap-y-4 rounded-md bg-white p-4"
      style="z-index: 1000;"
    >
      {@render children?.()}
    </div>
  </div>
{/if}
