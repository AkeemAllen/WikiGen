/**@type {import('./$types').PageLoad} */
import { BaseDirectory, exists, readTextFile, writeTextFile } from '@tauri-apps/api/fs';
import { wikis } from '../store';

export const prerender = true;
export const ssr = false;

export async function load({params}) {
    const wikiJsonExists = await exists("wikis.json", {dir: BaseDirectory.AppData});
    if (!wikiJsonExists) {
        await writeTextFile("wikis.json", JSON.stringify(wikis), {dir: BaseDirectory.AppData});
    }
    const contents = await readTextFile("wikis.json", {dir: BaseDirectory.AppData});
    wikis.set(JSON.parse(contents));
}