<script lang="ts">
  import { IconTrash } from "@tabler/icons-svelte";
  import { types } from "../../store/types";
</script>

<div class="grid grid-cols-3 gap-3">
  {#each $types as type}
    <div class="card flex flex-row items-center justify-between px-2 py-1">
      {type}
      <button class="btn rounded-sm p-2 hover:cursor-pointer hover:bg-gray-300">
        <IconTrash size={16} />
      </button>
    </div>
  {/each}
</div>
