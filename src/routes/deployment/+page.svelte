<script lang="ts">
  import { selectedWiki, user, wikis } from "../../store";
  import { Button } from "$lib/components/ui/button";
  import { onMount } from "svelte";
  import { Label } from "$lib/components/ui/label";
  import { Input } from "$lib/components/ui/input";
  import { Textarea } from "$lib/components/ui/textarea";
  import * as Card from "$lib/components/ui/card";
  import { load } from "@tauri-apps/plugin-store";
  import { toast } from "svelte-sonner";
  import { BaseDirectory, writeTextFile } from "@tauri-apps/plugin-fs";
  import LoadingModal from "$lib/components/modals/LoadingModal.svelte";
  import { invoke } from "@tauri-apps/api/core";
  import DeployWikiFinalStepsModal from "$lib/components/modals/DeployWikiFinalStepsModal.svelte";

  let creatingRepo = $state(false);
  let deployingWiki = $state(false);
  let deployWikiFinalStepsModal = $state(false);

  async function signOut() {
    const store = await load("store.json");
    await store.delete("token");
    await store.save();
    $user = {
      userName: "",
      avatarUrl: "",
      isConnected: false,
    };
  }

  async function createRepo() {
    const store = await load("store.json");
    const token = await store.get<string>("token");
    creatingRepo = true;
    toast.info("Creating repository...");
    //
    await fetch("https://wikigen-auth.fly.dev/create-repo", {
      method: "POST",
      body: JSON.stringify({
        token: token,
        wikiName: $selectedWiki.name,
      }),
      headers: {
        "Content-Type": "application/json",
      },
    })
      .then((res) => res.json())
      .then((res) => {
        if (res.status === 401) {
          toast.error("Token has expired. Relogin to deploy wiki");
          signOut();
        }
        return res;
      })
      .then(async (res) => {
        $selectedWiki.repo_url = `https://github.com/${$user.userName}/${$selectedWiki.name}`;
        $selectedWiki.settings.deployment_url = res.ssh_url;
        $wikis[$selectedWiki.name].repo_url = $selectedWiki.repo_url;
        $wikis[$selectedWiki.name].settings.deployment_url = res.ssh_url;
        await writeTextFile("wikis.json", JSON.stringify($wikis), {
          baseDir: BaseDirectory.AppData,
        }).catch((err) => {
          toast.error("Error writing wikis.json:", err);
          return;
        });
        toast.success("Repository created successfully!");
        creatingRepo = false;
      })
      .catch((err) => {
        toast.error("Error creating repository:", err);
        creatingRepo = false;
      });
  }

  async function deployWiki() {
    if ($selectedWiki.name === "") {
      toast.error("Wiki needs to be selected before deploying");
      return;
    }
    deployingWiki = true;

    invoke("deploy_wiki", {
      wikiName: $selectedWiki.name,
      sshUrl: $selectedWiki.settings.deployment_url,
    })
      .then(() => {
        toast.success("Wiki Preparation Complete!");
        deployingWiki = false;
        deployWikiFinalStepsModal = true;
      })
      .catch((err) => {
        toast.error(`Error while deploying wiki!: ${err}`);
        deployingWiki = false;
      });
  }
</script>

<LoadingModal
  message="Preparing Wiki For Deployment"
  bind:loading={deployingWiki}
/>

<DeployWikiFinalStepsModal bind:deployWikiFinalStepsModal />

<Card.Root class="mx-5 mt-5">
  <Card.Content>
    {#if $selectedWiki.repo_url === "" || $selectedWiki.repo_url === undefined}
      <Button
        class="cursor-pointer"
        disabled={!$user.isConnected || creatingRepo}
        onclick={createRepo}>Create Repository</Button
      >
    {:else}
      <p class="mb-4">Associated Repository: {$selectedWiki.repo_url}</p>
      <Button
        class="cursor-pointer"
        disabled={!$user.isConnected || creatingRepo}
        onclick={deployWiki}>Deploy Wiki</Button
      >
    {/if}
    {#if !$user.isConnected}
      <p class="mt-2">Sign in to github first to create a repository.</p>
    {/if}
  </Card.Content>
</Card.Root>
