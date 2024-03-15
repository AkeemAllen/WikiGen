<script lang="ts">
  import { wikis } from "../../store";

  let wikiName = "";
  let wikiDescription = "";
  async function createWiki() {
    if (wikiName === "" || wikiDescription === "") {
      return;
    }
    if ($wikis[wikiName] !== undefined) {
      return;
    }
    wikis.update((w) => {
      w[wikiName] = {
        name: wikiName,
        description: wikiDescription,
      };
      return w;
    });
  }
</script>

<div class="flex flex-col gap-4 mx-auto max-w-60">
  <p class="mt-4 font-bold text-xl w-1/2">New Wiki</p>
  <input class="p-3" placeholder="Wiki Name" bind:value={wikiName} />
  <input
    class="p-3"
    placeholder="Wiki Description"
    bind:value={wikiDescription}
  />
  <button
    disabled={wikiDescription === "" || wikiName === ""}
    class="btn btn-md w-30 bg-blue-600"
    on:click={createWiki}>Create Wiki</button
  >
</div>
