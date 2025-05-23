/**@type {import('./$types').PageLoad} */
import {
  BaseDirectory,
  create,
  exists,
  mkdir,
  readTextFile,
  writeTextFile,
} from "@tauri-apps/plugin-fs";
import { wikis } from "../store";
import { getToastStore } from "@skeletonlabs/skeleton";
import { ToastType, getToastSettings } from "$lib/utils/toasts";

export const prerender = true;
export const ssr = false;

export async function load({ params }: { params: { slug: string } }) {
  // These lines ensure that the appData directory exists on initial application launch.
  // Would prefer a more ideal way to do this, but this is the best I could come up with.
  //
  // The directory_existence folder just serves as a placeholder to ensure the appData directory is created.
  const appDataDirectoryExists = await exists("directory_existence", {
    baseDir: BaseDirectory.AppData,
  });
  if (!appDataDirectoryExists) {
    await mkdir("directory_existence", {
      baseDir: BaseDirectory.AppData,
    });
  }

  const wikiJsonExists = await exists("wikis.json", {
    baseDir: BaseDirectory.AppData,
  });
  if (!wikiJsonExists) {
    await writeTextFile("wikis.json", JSON.stringify(wikis), {
      baseDir: BaseDirectory.AppData,
    }).catch((err) => {
      console.log(err);
    });
  }
  const contents = await readTextFile("wikis.json", {
    baseDir: BaseDirectory.AppData,
  }).catch((err) => {
    console.log(err);
  });
  wikis.set(JSON.parse(contents as string));
}
