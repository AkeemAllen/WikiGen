<script lang="ts">
  import BaseModal from "$lib/components/BaseModal.svelte";
  import Button from "$lib/components/Button.svelte";
  import TextInput from "$lib/components/TextInput.svelte";
  import { getToastStore } from "@skeletonlabs/skeleton";
  import { invoke } from "@tauri-apps/api";
  import { selectedWiki } from "../../store";

  const toastStore = getToastStore();

  let routeName: string = "";
  let newRouteModalOpen: boolean = false;
  let loading = false;

  async function createNewRoute() {
    loading = true;
    await invoke("create_new_route", {
      wikiName: $selectedWiki.name,
      routeName,
    }).then(() => {
      loading = false;
      toastStore.trigger({
        message: "New Route Added",
      });
    });
  }
</script>

<div class="w-40">
  <Button title="Create New Route" onClick={() => (newRouteModalOpen = true)} />
</div>
<BaseModal bind:open={newRouteModalOpen}>
  <TextInput bind:value={routeName} label="Route Name" />
  <Button
    title="Save New Route"
    onClick={() => {
      createNewRoute();
      newRouteModalOpen = false;
    }}
  />
</BaseModal>
