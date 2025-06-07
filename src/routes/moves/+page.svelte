<script lang="ts">
  import { Button } from "$lib/components/ui/button/index";
  import { selectedWiki } from "../../store";
  import { moveList, type Move } from "../../store/moves";
  import { types as pokemonTypes } from "../../store/types";
  import { db } from "../../store/db";
  import { FALSE, TRUE } from "$lib/utils/CONSTANTS";

  import { cloneDeep } from "$lib/utils/cloneDeep";
  import capitalizeWords from "$lib/utils/capitalizeWords";
  import isEqual from "$lib/utils/isEqual";
  import objectIsEmpty from "$lib/utils/objectIsEmpty";
  import { generatePokemonPages } from "$lib/utils/generators";
  import * as Card from "$lib/components/ui/card/index.js";
  import Autocomplete from "$lib/components/ui/Autocomplete.svelte";
  import SaveIcon from "@lucide/svelte/icons/save";
  import { Input } from "$lib/components/ui/input/index.js";
  import { Label } from "$lib/components/ui/label";
  import * as Select from "$lib/components/ui/select/index.js";
  import { Checkbox } from "$lib/components/ui/checkbox/index.js";
  import * as Dialog from "$lib/components/ui/dialog/index.js";
  import { toast } from "svelte-sonner";

  let moveSearch: [number, string] = $state([0, ""]);
  let moveSearchOpen = $state(false);
  let searchingMoves = $state("");

  let newMoveModalOpen: boolean = $state(false);
  let newMove: Move = $state({
    name: "",
    damage_class: "status",
    type: "normal",
  } as Move);
  let move: Move = $state({} as Move);
  let originalMoveDetails: Move = $state({} as Move);

  let moveListOptions = $state(
    $moveList.map(([id, name]) => ({
      label: name,
      value: id,
    })),
  );

  let options = $derived(
    moveListOptions
      .filter((move) =>
        move.label.toLowerCase().includes(searchingMoves.toLowerCase()),
      )
      .slice(0, 8),
  );

  type MoveLearner = {
    pokemonId: number;
    name: string;
    learn_method: string;
    level_learned: number;
  };
  let pokemonWhoCanLearnMove: MoveLearner[] = $state([]);

  async function gatherPokemonWhoCanLearnMove() {
    return await $db
      .select<MoveLearner[]>(
        `SELECT DISTINCT pokemon as pokemonId, p.name, learn_method, level_learned
      FROM pokemon_movesets
      INNER JOIN pokemon p on p.id = pokemon
      WHERE move=$1;`,
        [move.id],
      )
      .then((res) => {
        pokemonWhoCanLearnMove = res;
      })
      .catch((err) => {
        toast.error(`Error getting Pokemon who can learn move: ${err}`);
      });
  }

  async function getMove() {
    let retrievedMove = $moveList.find(([_, name]) => name === moveSearch[1]);

    if (!retrievedMove) {
      toast.error("Move not found!");
      return;
    }

    await $db
      .select<Move[]>("SELECT * FROM moves WHERE id = $1;", [moveSearch[0]])
      .then(async (res) => {
        move = res[0];
        originalMoveDetails = cloneDeep(move);
      })
      .then(async () => {
        await gatherPokemonWhoCanLearnMove();
      })
      .catch((err) => {
        toast.error(`Error getting moves: ${err}`);
      });
  }

  async function saveMoveChanges() {
    await $db
      .execute(
        "UPDATE moves SET power=$1,accuracy=$2,pp=$3,type=$4,damage_class=$5,machine_name=$6,is_modified=$7 WHERE id = $8;",
        [
          move.power,
          move.accuracy,
          move.pp,
          move.type,
          move.damage_class,
          move.machine_name,
          move.is_modified,
          move.id,
        ],
      )
      .then(() => {
        originalMoveDetails = cloneDeep(move);
        generatePokemonPages(
          pokemonWhoCanLearnMove.map((p) => p.pokemonId),
          $selectedWiki.name,
        )
          .then(() => {
            toast.success(`Generated Pages for Pokemon with ${move.name}!`);
          })
          .catch((err) => {
            toast.error(`Error generating Pokemon pages: ${err}`);
          });
      })
      .catch(() => {
        toast.error("Error saving move changes!");
      });
  }

  async function createNewMove() {
    if (newMove.power === 0) {
      newMove.power = null;
    }
    if (newMove.accuracy === 0) {
      newMove.accuracy = null;
    }
    await $db
      .execute(
        `INSERT INTO moves (name, power, accuracy, pp, type, damage_class, machine_name)
        VALUES ($1, $2, $3, $4, $5, $6, $7);`,
        [
          newMove.name.toLowerCase(),
          newMove.power,
          newMove.accuracy,
          newMove.pp,
          newMove.type,
          newMove.damage_class,
          newMove.machine_name,
        ],
      )
      .then((res) => {
        toast.success("Move created!");
        newMoveModalOpen = false;
        $moveList.push([res.lastInsertId as number, newMove.name]);

        newMove = {
          damage_class: "status",
          type: "normal",
        } as Move;

        moveListOptions = $moveList.map(([id, name]) => ({
          label: name,
          value: id,
        }));
      })
      .catch((err) => {
        toast.error(`Error creating new move: ${err}`);
      });
  }

  function setModified(e: any) {
    move.is_modified = e.target?.checked ? TRUE : FALSE;
  }
</script>

<Dialog.Root class="w-[30rem]" bind:open={newMoveModalOpen}>
  <Dialog.Content>
    <Dialog.Header>
      <Dialog.Title>Create New Move</Dialog.Title>
    </Dialog.Header>
    <form onsubmit={createNewMove} class="flex flex-col gap-4">
      <div>
        <Label for="name" class="block text-sm font-medium mb-2 text-gray-700"
          >New Move</Label
        >
        <Input
          bind:value={newMove.name}
          oninput={(e: any) => {
            newMove.name = e.target.value.toLowerCase().replaceAll(" ", "-");
          }}
        />
      </div>
      <div>
        <Label for="power" class="text-sm font-medium text-slate-700 mb-2 block"
          >Power</Label
        >
        <Input id="power" type="number" bind:value={newMove.power} max={255} />
      </div>
      <div>
        <Label for="type" class="text-sm font-medium text-slate-700 mb-2 block"
          >Type</Label
        >
        <Select.Root type="single" bind:value={newMove.type}>
          <Select.Trigger id="type" class="w-full">
            {capitalizeWords(newMove.type)}
          </Select.Trigger>
          <Select.Content>
            {#each $pokemonTypes as type}
              <Select.Item value={type} label={type}>
                {capitalizeWords(type)}
              </Select.Item>
            {/each}
          </Select.Content>
        </Select.Root>
      </div>
      <div>
        <Label
          for="accuracy"
          class="text-sm font-medium text-slate-700 mb-2 block">Accuracy</Label
        >
        <Input
          id="accuracy"
          type="number"
          bind:value={newMove.accuracy}
          max={100}
        />
      </div>
      <div>
        <Label for="pp" class="text-sm font-medium text-slate-700 mb-2 block"
          >PP</Label
        >
        <!-- Highest PP newMove is 40, but setting it to 100 for future proofing -->
        <Input id="pp" type="number" bind:value={newMove.pp} max={100} />
      </div>
      <div>
        <Label
          for="damage-class"
          class="text-sm font-medium text-slate-700 mb-2 block"
          >Damage Class</Label
        >
        <Select.Root type="single" bind:value={newMove.damage_class}>
          <Select.Trigger id="damage-class" class="w-full">
            {capitalizeWords(newMove.damage_class)}
          </Select.Trigger>
          <Select.Content>
            {#each [{ label: "status", value: "status" }, { label: "physical", value: "physical" }, { label: "special", value: "special" }] as damageClass}
              <Select.Item value={damageClass.value} label={damageClass.label}>
                {capitalizeWords(damageClass.label)}
              </Select.Item>
            {/each}
          </Select.Content>
        </Select.Root>
      </div>
      <div>
        <Label
          for="machine-name"
          class="text-sm font-medium text-slate-700 mb-2 block"
          >Machine Name</Label
        >
        <Input id="machine-name" type="text" bind:value={move.machine_name} />
      </div>
      <Dialog.Footer>
        <Button type="submit" class="mt-5" disabled={newMove.name === ""}>
          Create Move
        </Button>
      </Dialog.Footer>
    </form>
  </Dialog.Content>
</Dialog.Root>

<Card.Root class="mx-5 mt-5">
  <Card.Content class="flex flex-row gap-3">
    <Autocomplete
      open={moveSearchOpen}
      value={moveSearch[1]}
      bind:searcher={searchingMoves}
      {options}
      placeholder="Search Moves"
      onSelect={(option) => {
        moveSearch = [option.value, option.label];
        getMove();
      }}
      class="w-[15rem]"
    />
    <div class="flex flex-row justify-between w-full">
      <Button
        class="cursor-pointer"
        onclick={saveMoveChanges}
        disabled={isEqual(move, originalMoveDetails)}
      >
        <SaveIcon />
        Save Changes</Button
      >
      <Button
        variant="outline"
        class="cursor-pointer"
        onclick={() => (newMoveModalOpen = true)}
      >
        Create New Move</Button
      >
    </div>
  </Card.Content>
</Card.Root>

{#if !objectIsEmpty(move)}
  <Card.Root class="mx-5 mt-5">
    <Card.Content>
      <div class="grid grid-cols-2 gap-5">
        <div>
          <Label
            for="power"
            class="text-sm font-medium text-slate-700 mb-2 block">Power</Label
          >
          <Input id="power" type="number" bind:value={move.power} max={255} />
        </div>
        <div>
          <Label
            for="type"
            class="text-sm font-medium text-slate-700 mb-2 block">Type</Label
          >
          <Select.Root type="single" bind:value={move.type}>
            <Select.Trigger id="type" class="w-full">
              {capitalizeWords(move.type)}
            </Select.Trigger>
            <Select.Content>
              {#each $pokemonTypes as type}
                <Select.Item value={type} label={type}>
                  {capitalizeWords(type)}
                </Select.Item>
              {/each}
            </Select.Content>
          </Select.Root>
        </div>
        <div>
          <Label
            for="accuracy"
            class="text-sm font-medium text-slate-700 mb-2 block"
            >Accuracy</Label
          >
          <Input
            id="accuracy"
            type="number"
            bind:value={move.accuracy}
            max={100}
          />
        </div>
        <div>
          <Label for="pp" class="text-sm font-medium text-slate-700 mb-2 block"
            >PP</Label
          >
          <!-- Highest PP move is 40, but setting it to 100 for future proofing -->
          <Input id="pp" type="number" bind:value={move.pp} max={100} />
        </div>
        <div>
          <Label
            for="damage-class"
            class="text-sm font-medium text-slate-700 mb-2 block"
            >Damage Class</Label
          >
          <Select.Root type="single" bind:value={move.damage_class}>
            <Select.Trigger id="damage-class" class="w-full">
              {capitalizeWords(move.damage_class)}
            </Select.Trigger>
            <Select.Content>
              {#each [{ label: "status", value: "status" }, { label: "physical", value: "physical" }, { label: "special", value: "special" }] as damageClass}
                <Select.Item
                  value={damageClass.value}
                  label={damageClass.label}
                >
                  {capitalizeWords(damageClass.label)}
                </Select.Item>
              {/each}
            </Select.Content>
          </Select.Root>
        </div>
        <div>
          <Label
            for="machine-name"
            class="text-sm font-medium text-slate-700 mb-2 block"
            >Machine Name</Label
          >
          <Input id="machine-name" type="text" bind:value={move.machine_name} />
        </div>
      </div>
      {#if !move.is_new}
        <div class="flex flex-row space-x-2 items-center mt-5">
          <Checkbox
            id="is-modified"
            bind:checked={move.is_modified}
            onchange={setModified}
            class="text-sm font-medium leading-6 text-gray-900"
          />
          <Label
            for="is-modified"
            class="block text-sm font-medium leading-6 text-gray-900"
          >
            Mark Move as Modified
          </Label>
        </div>
      {/if}
    </Card.Content>
  </Card.Root>
{/if}
