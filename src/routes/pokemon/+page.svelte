<script lang="ts">
    import { TabGroup, Tab } from "@skeletonlabs/skeleton";
    import {Command} from "@tauri-apps/api/shell";
    import {invoke} from "@tauri-apps/api/tauri";

    let tabSet: number = 0

    let pokemonName = ""
    let message = ""
    async function greet() { 
        message = await invoke("greet", {pokemonName})
    }
</script>

<TabGroup class="mt-4">
    <Tab bind:group={tabSet} name="pokemon" value={0}>Pokemon</Tab>
    <Tab bind:group={tabSet} name="prepare-pokemon-data" value={1}>Prepare Data</Tab>
    <svelte:fragment slot="panel">
        {#if tabSet === 0} 
            <div>
                <input id="greet-input" type="text" placeholder="Pokemon Name" bind:value="{pokemonName}" class="ml-2"/>
                <button on:click="{greet}" class="btn">Search</button>
                <p>{message}</p>
            </div>
        {/if}
    </svelte:fragment>
</TabGroup>