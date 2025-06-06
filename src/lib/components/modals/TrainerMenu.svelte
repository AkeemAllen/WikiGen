<script lang="ts">
  import IconDots from "@tabler/icons-svelte/icons/dots";
  import { popup } from "@skeletonlabs/skeleton";
  import ImageIcon from "@lucide/svelte/icons/image";
  import SplitIcon from "@lucide/svelte/icons/split";
  import ArrowLeftRightIcon from "@lucide/svelte/icons/arrow-left-right";

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
    positionModalOpen = $bindable(false),
  }: Props = $props();
</script>

<!-- <button
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
</div> -->
<div class="flex flex-row gap-3">
  <button class="rounded-md p-1 mr-2 hover:cursor-pointer hover:bg-slate-200">
    <ImageIcon class="text-slate-400 size-4 self-center" />
  </button>
  <button class="rounded-md p-1 mr-2 hover:cursor-pointer hover:bg-slate-200">
    <SplitIcon class="text-slate-400 size-4 self-center" />
  </button>
  <button class="rounded-md p-1 mr-2 hover:cursor-pointer hover:bg-slate-200">
    <ArrowLeftRightIcon class="text-slate-400 size-4 self-center" />
  </button>
</div>
