<script lang="ts">
import type { DataHandler } from "@vincjo/datatables";
  interface Props {
    handler: DataHandler;
  }

  let { handler }: Props = $props();
let pageNumber = $derived(handler.getPageNumber());
let pageCount = $derived(handler.getPageCount());
let pages = $derived(handler.getPages({ ellipsis: true }));
</script>

<!-- Desktop buttons -->
<section class="variant-ghost-surface btn-group hidden h-10 bg-white lg:block">
  <button
    type="button"
    class="hover:variant-soft-primary"
    class:disabled={$pageNumber === 1}
    onclick={() => handler.setPage("previous")}
  >
    ←
  </button>
  {#each $pages as page}
    <button
      type="button"
      class="hover:variant-soft-primary"
      class:active={$pageNumber === page}
      class:ellipse={page === null}
      onclick={() => handler.setPage(page)}
    >
      {page ?? "..."}
    </button>
  {/each}
  <button
    type="button"
    class="hover:variant-soft-primary"
    class:disabled={$pageNumber === $pageCount}
    onclick={() => handler.setPage("next")}
  >
    →
  </button>
</section>

<!-- Mobile buttons -->
<section class="lg:hidden">
  <button
    type="button"
    class="variant-ghost-surface btn mb-2 mr-2 hover:variant-soft-primary"
    class:disabled={$pageNumber === 1}
    onclick={() => handler.setPage("previous")}
  >
    ←
  </button>
  <button
    type="button"
    class="variant-ghost-surface btn mb-2 hover:variant-soft-primary"
    class:disabled={$pageNumber === $pageCount}
    onclick={() => handler.setPage("next")}
  >
    →
  </button>
</section>
