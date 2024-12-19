<script lang="ts">
  interface Props {
    name?: string;
    route?: string;
    active?: boolean;
    icon?: import('svelte').Snippet;
  }

  let {
    name = "",
    route = "",
    active = false,
    icon
  }: Props = $props();
</script>

<a
  href={route}
  id="sidebar-left"
  class={`p-1 border-0 w-full hover:cursor-pointer  ${active ? "bg-red-100" : "bg-transparent hover:bg-zinc-100"} rounded-md text-sm text-gray-500 space-between`}
>
  <div class="flex items-center gap-3">
    <div class="p-1 rounded-md">
      {@render icon?.()}
    </div>
    <span class={`${active ? "text-red-500" : "text-black"}`}>
      {name}
    </span>
  </div>
</a>
