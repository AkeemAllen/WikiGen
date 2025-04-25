<script lang="ts">
  import IconDots from "@tabler/icons-svelte/icons/dots";
  import { popup } from "@skeletonlabs/skeleton";
  interface Props {
    index: any;
    trainerToUpdate: string;
    trainerName: string;
    spriteModalOpen?: boolean;
    trainerVersionsModalOpen?: boolean;
    positionModalOpen?: boolean;
  }

  let {
    index,
    trainerToUpdate = $bindable(),
    trainerName,
    spriteModalOpen = $bindable(false),
    trainerVersionsModalOpen = $bindable(false),
    positionModalOpen = $bindable(false)
  }: Props = $props();
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
    onclick={() => {
      spriteModalOpen = true;
      trainerToUpdate = trainerName;
    }}>Add Sprite</button
  >
  <button
    class="w-full rounded-md p-2 text-left text-sm hover:bg-slate-300"
    onclick={() => {
      trainerVersionsModalOpen = true;
      trainerToUpdate = trainerName;
    }}>Trainer Versions</button
  >
  <button
    class="w-full rounded-md p-2 text-left text-sm hover:bg-slate-300"
    onclick={() => {
      positionModalOpen = true;
      trainerToUpdate = trainerName;
    }}>Change Position</button
  >
</div>
