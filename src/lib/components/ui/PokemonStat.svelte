<script lang="ts">
  import { Label } from "$lib/components/ui/label/index.js";
  import { Slider } from "$lib/components/ui/slider/index.js";
  import { Input } from "$lib/components/ui/input/index.js";

  type Props = {
    value: number;
    label: string;
  };
  let { value = $bindable(), label }: Props = $props();
</script>

<div class="flex flex-col">
  <Label for={label} class="text-sm font-medium text-slate-700 mb-2 block"
    >{label}</Label
  >
  <div class="flex flex-row gap-2" id={`${label}`}>
    <Slider type="single" step={1} max={255} bind:value />
    <Input type="number" min={1} max={255} bind:value class="w-24" />
  </div>
</div>
