<script>
    import { invoke } from '@tauri-apps/api/tauri';
    let name = ""
    let greeting = ""

    async function greet() {
        greeting = await invoke('greet', { name })
        
    }
    </script>

    <div>
        <input id="greet-input" placeholder="Enter your name" bind:value={name} />
        <button on:click={greet}>Greet</button>
        <p>{greeting}</p>
    </div>
    <div>
        Something
    </div>