<script lang="ts">
  import { selectedWiki } from "../store";
</script>

{#if $selectedWiki.name === ""}
  <p class="mt-4">Select or Create a Wiki using the Select Wiki Button</p>
{/if}

<!-- Improve Page design (Just stick with original design for now) -->
<!-- Add ability to create wiki -->
<!-- Read wiki information from json files -->
