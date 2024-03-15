/**@type {import('./$types').PageLoad} */
import { BaseDirectory, readTextFile } from '@tauri-apps/api/fs';
import { wikis } from '../store';

export const prerender = true;
export const ssr = false;

export async function load({params}) {
    const contents = await readTextFile("data/wikis.json", {dir: BaseDirectory.AppData});
    wikis.set(JSON.parse(contents));
}