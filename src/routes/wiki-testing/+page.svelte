<script lang="ts">
  import Button from "$lib/components/Button.svelte";
  import { appDataDir } from "@tauri-apps/api/path";
  import { invoke } from "@tauri-apps/api/tauri";
  import { selectedWiki } from "../../store";

  async function spawnProcess() {
    const appData = await appDataDir();
    console.log(`${appData}${$selectedWiki.name}/dist/mkdocs.yml`);
    await invoke("spawn_mkdocs_process", {
      mkdocsFilePath: `${appData}${$selectedWiki.name}/dist/mkdocs.yml`,
    }).then((response) => {
      console.log(response);
    });
  }
</script>

<div class="mt-6 ml-2">
  <Button title="Start Test Server" onClick={spawnProcess} />
</div>
