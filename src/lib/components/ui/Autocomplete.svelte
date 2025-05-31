<script lang="ts">
  import { tick } from "svelte";
  import * as Command from "$lib/components/ui/command/index.js";
  import * as Popover from "$lib/components/ui/popover/index.js";
  import { Label } from "$lib/components/ui/label/index.js";
  import { Button } from "$lib/components/ui/button/index.js";
  import ChevronsUpDownIcon from "@lucide/svelte/icons/chevrons-up-down";

  type Props = {
    open: boolean;
    searcher: string;
    placeholder: string;
    triggerRef: HTMLButtonElement;
    value: any;
    onSelect: (option: { label: string; value: any }) => void;
    options: { label: string; value: any }[];
    label?: string;
    class?: string;
  };

  let {
    open = $bindable(),
    searcher = $bindable(),
    value = $bindable(),
    class: className,
    label,
    onSelect,
    triggerRef,
    options,
  }: Props = $props();

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
          {value || "Select Pokemon"}
          <ChevronsUpDownIcon class="opacity-50" />
        </Button>
      </div>
    {/snippet}
  </Popover.Trigger>
  <Popover.Content class="w-full p-0">
    <Command.Root shouldFilter={false}>
      <Command.Input placeholder="Search Pokemon" bind:value={searcher} />
      <Command.List>
        <Command.Empty>Such Empty</Command.Empty>
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
