<script lang="ts">
  import type { DataHandler } from "@vincjo/datatables";
  interface Props {
    handler: DataHandler;
  }

  let { handler }: Props = $props();
  const rowCount = handler.getRowCount();
</script>

<aside class="text-sm leading-8 mr-6">
  {#if $rowCount.total > 0}
    <b>{$rowCount.start}</b>
    - <b>{$rowCount.end}</b>
    / <b>{$rowCount.total}</b>
  {:else}
    No entries found
  {/if}
</aside>
