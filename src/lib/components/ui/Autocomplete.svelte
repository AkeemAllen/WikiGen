<script lang="ts">
  import { tick } from "svelte";
  import * as Command from "$lib/components/ui/command/index.js";
  import * as Popover from "$lib/components/ui/popover/index.js";
  import { Label } from "$lib/components/ui/label/index.js";
  import { Button } from "$lib/components/ui/button/index.js";
  import ChevronsUpDownIcon from "@lucide/svelte/icons/chevrons-up-down";
  import capitalizeWords from "$lib/utils/capitalizeWords";

  type Props = {
    open: boolean;
    searcher: string;
    placeholder: string;
    value: any;
    onSelect: (option: { label: string; value: any }) => void;
    options: { label: string; value: any }[];
    label?: string;
    class?: string;
    creationEnabled?: boolean;
  };

  let {
    open = $bindable(),
    searcher = $bindable(),
    value = $bindable(),
    class: className,
    placeholder,
    label,
    onSelect,
    options,
    creationEnabled = false,
  }: Props = $props();

  let triggerRef = $state<HTMLButtonElement>(null!);

  let optionsPresent = $derived.by(() => {
    let option = options.find((option) =>
      option.label.toLowerCase().includes(searcher.toLowerCase()),
    );
    return option !== undefined;
  });

  // We want to refocus the trigger button when the user selects
  // an item from the list so users can continue navigating the
  // rest of the form with the keyboard.
  function closeAndFocusTrigger() {
    open = false;
    tick().then(() => {
      triggerRef.focus();
    });
  }
</script>

<Popover.Root bind:open>
  <Popover.Trigger bind:ref={triggerRef}>
    {#snippet child({ props })}
      <div>
        {#if label}
          <Label
            for={label}
            class="text-sm font-medium text-slate-700 mb-2 block">{label}</Label
          >
        {/if}
        <Button
          id={label || ""}
          variant="outline"
          {...props}
          class={className}
          role="combobox"
          aria-expanded={open}
        >
          {capitalizeWords(value) || placeholder}
          <ChevronsUpDownIcon class="opacity-50" />
        </Button>
      </div>
    {/snippet}
  </Popover.Trigger>
  <Popover.Content class="w-full p-0">
    <Command.Root shouldFilter={false}>
      <Command.Input {placeholder} bind:value={searcher} />
      <Command.List>
        {#if creationEnabled && !optionsPresent}
          <Command.Item
            value={searcher}
            onSelect={() => {
              onSelect({ label: searcher, value: searcher });
              closeAndFocusTrigger();
            }}
          >
            No Results Found. Create one?
          </Command.Item>
        {:else}
          <Command.Empty>Such Empty</Command.Empty>
        {/if}
        {#each options as option}
          <Command.Item
            value={option.label}
            onSelect={() => {
              onSelect(option);
              closeAndFocusTrigger();
            }}
          >
            {option.label}
          </Command.Item>
        {/each}
      </Command.List>
    </Command.Root>
  </Popover.Content>
</Popover.Root>
