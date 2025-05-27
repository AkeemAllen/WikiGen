<script lang="ts">
  import * as Dialog from "../ui/dialog";
  import LoaderCircleIcon from "@lucide/svelte/icons/loader-circle";

  type Props = {
    loading?: boolean;
    message?: string;
  };

  let { loading = $bindable(false), message = "" }: Props = $props();
</script>

<Dialog.Root bind:open={loading}>
  <Dialog.Content class="sm:max-w-[425px]">
    <Dialog.Header class="flex flex-row gap-4">
      <LoaderCircleIcon class="animate-spin" />
      {message}
    </Dialog.Header>
  </Dialog.Content>
</Dialog.Root>
