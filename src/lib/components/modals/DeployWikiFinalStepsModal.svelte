<script lang="ts">
  import * as Dialog from "$lib/components/ui/dialog";
  import { selectedWiki, user } from "../../../store";
  import { appDataDir } from "@tauri-apps/api/path";
  import { type } from "@tauri-apps/plugin-os";

  let osType = $state("");
  let { deployWikiFinalStepsModal = $bindable(false) } = $props();

  let mkdocsFilePath: Promise<string> = $derived.by(async () => {
    const appData = await appDataDir();
    let mkdocsFilePath = `${appData}/${$selectedWiki.name}/dist`;
    osType = type();
    if (osType === "windows") {
      mkdocsFilePath = mkdocsFilePath.replace(/\//g, "\\");
    } else if (osType === "macos") {
      mkdocsFilePath = mkdocsFilePath.replace(/\s/g, "\\ ");
    }
    return mkdocsFilePath;
  });
</script>

<Dialog.Root bind:open={deployWikiFinalStepsModal}>
  <Dialog.Content class="min-w-[40rem]">
    <Dialog.Header>Final Deploy Stesp</Dialog.Header>
    <div class="grid gap-3">
      <p>
        1.
        {#if osType === "windows"}
          Press Windows Start Key, type Terminal, and open it
        {:else if osType === "macos"}
          Open spotlight search, type Terminal, and open it
        {/if}
      </p>
      <h2 class="text-md font-semibold leading-3 text-gray-900">
        Copy and paste the below commands
      </h2>
      <p>
        2. Navigate to the Wiki
        <br />
        <span class="code font-semibold">
          cd {#await mkdocsFilePath then filepath}
            {filepath}
          {/await}</span
        >
      </p>
      <p>
        3. Update the main branch
        <br />
        <span class="code font-semibold"> git push -u origin main</span>
      </p>
      <p>
        4. Deploy the Wiki
        <br />
        <span class="code font-semibold"> mkdocs gh-deploy</span>
      </p>
      <p>
        4. Once done, your docs should be available at the below URL. Note that
        it may take a few minutes to load.
        <br />
        <a
          href={`https://${$user.userName.toLowerCase()}.github.io/${$selectedWiki.name}/`}
          target="_blank"
        >
          <span class="code font-semibold"
            >https://{$user.userName.toLowerCase()}.github.io/{$selectedWiki.name}/</span
          >
        </a>
      </p>
    </div>
  </Dialog.Content>
</Dialog.Root>
