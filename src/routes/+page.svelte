<script lang="ts">
    import {wikis, selectedWiki} from "../store" 
</script>

<div class="mt-4">Select A Wiki</div>
<div class="gird grid-cols-4">
    {#each Object.entries($wikis) as [_, value]}
        <button on:click={() => $selectedWiki = value} class="btn btn-primary">
            {value.name}</button>
    {/each}
</div>

