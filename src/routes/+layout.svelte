<script>
    import { AppBar, AppShell } from "@skeletonlabs/skeleton";
    import "../app.pcss";

    import {wikis, selectedWiki} from "../store"
</script>

<AppShell>
    <svelte:fragment slot="header">
        <AppBar>
           <a href="/">WikiGen</a>
        </AppBar>
    </svelte:fragment>
    <svelte:fragment slot="sidebarLeft">
        {#if $selectedWiki.name !== ""}
            <div class="container mx-auto p-4 grid gap-3">
                <a href="/pokemon" id="sidebar-left" class="hidden lg:block btn border-0 hover:cursor-pointer rounded-md p-2 bg-blue-200 w-40 items-center">
                    Pokemon
                </a>
            </div>        
        {/if}
    </svelte:fragment>
    <slot/>
</AppShell>