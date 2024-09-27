/**@type {import('./$types').PageLoad} */
import {
  BaseDirectory,
  createDir,
  exists,
  readTextFile,
  writeTextFile,
} from "@tauri-apps/api/fs";
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
    dir: BaseDirectory.AppData,
  });
  if (!appDataDirectoryExists) {
    await createDir("directory_existence", {
      dir: BaseDirectory.AppData,
      recursive: true,
    });
  }

  const wikiJsonExists = await exists("wikis.json", {
    dir: BaseDirectory.AppData,
  });
  if (!wikiJsonExists) {
    await writeTextFile("wikis.json", JSON.stringify(wikis), {
      dir: BaseDirectory.AppData,
    }).catch((err) => {
      getToastStore().trigger(
        getToastSettings(ToastType.ERROR, `Error writing wikis.json: ${err}`),
      );
    });
  }
  const contents = await readTextFile("wikis.json", {
    dir: BaseDirectory.AppData,
  }).catch((err) => {
    getToastStore().trigger(
      getToastSettings(ToastType.ERROR, `Error reading wikis.json: ${err}`),
    );
  });
  wikis.set(JSON.parse(contents as string));
}
