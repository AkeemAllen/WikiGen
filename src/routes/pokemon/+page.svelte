<script lang="ts">
    import { TabGroup, Tab } from "@skeletonlabs/skeleton";
    import {Command} from "@tauri-apps/api/shell";
    let tabSet: number = 0

    let message = ""
    async function executeCommand() {
        const command = Command.sidecar('bin/python/test')
        const output = await command.execute()
        const {stdout, stderr} = output

        message = stdout
    }
</script>

<TabGroup class="mt-4">
    <Tab bind:group={tabSet} name="pokemon" value={0}>Pokemon</Tab>
    <Tab bind:group={tabSet} name="prepare-pokemon-data" value={1}>Prepare Data</Tab>
    <svelte:fragment slot="panel">
        {#if tabSet === 0} 
            <div>
                <input type="text" placeholder="Pokemon Name" />
                <button on:click={executeCommand} class="btn">Search</button>
                <div>{message}</div>
            </div>
        {/if}
    </svelte:fragment>
</TabGroup>