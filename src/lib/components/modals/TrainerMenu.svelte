<script lang="ts">
import { IconDots } from "@tabler/icons-svelte";
import { popup } from "@skeletonlabs/skeleton";
export let index;
export let trainerToUpdate: string;
export let trainerName: string;
export let spriteModalOpen: boolean = false;
export let trainerVersionsModalOpen: boolean = false;
export let positionModalOpen: boolean = false;
</script>

<button
  class="hover:cursor-pointer"
  use:popup={{
            event: "click",
            target: "trainerMenu" + index,
            placement: "right",
          }}
>
  <IconDots size={20} color="gray" />
</button>
<div class="card z-10 w-44 bg-white p-2" data-popup="trainerMenu{index}">
  <button
    class="w-full rounded-md p-2 text-left text-sm hover:bg-slate-300"
    on:click={() => {
                    spriteModalOpen = true;
                    trainerToUpdate = trainerName;
                }}
    >Add Sprite</button
  >
  <button
    class="w-full rounded-md p-2 text-left text-sm hover:bg-slate-300"
    on:click={() => {
                    trainerVersionsModalOpen = true;
                    trainerToUpdate = trainerName;
                }}
    >Trainer Versions</button
  >
  <button
    class="w-full rounded-md p-2 text-left text-sm hover:bg-slate-300"
    on:click={() => {
                    positionModalOpen = true;
                    trainerToUpdate = trainerName;
                }}
    >Change Position</button
  >
</div>
