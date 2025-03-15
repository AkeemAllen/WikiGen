<script lang="ts">
  export let name = "";
  export let route = "";
  export let active = false;
</script>

<a
  href={route}
  id="sidebar-left"
  class={`p-1 border-0 w-full hover:cursor-pointer  ${active ? "bg-indigo-100" : "bg-transparent hover:bg-gray-100"} rounded-md text-sm text-gray-500 space-between`}
>
  <div class="flex items-center gap-3">
    <div class="p-1 rounded-md">
      <slot name="icon" />
    </div>
    <span class={`${active ? "text-indigo-500" : "text-black"}`}>
      {name}
    </span>
  </div>
</a>
