<script lang="ts">
  export let name = "";
  export let route = "";
  export let active = false;
</script>

<a
  href={route}
  id="sidebar-left"
  class={`p-2 border-0 w-[12rem] hover:cursor-pointer hover:bg-slate-100 ${active ? "bg-indigo-100 hover:bg-indigo-100" : "bg-white"} rounded-md text-sm text-gray-500 space-between`}
>
  <div class="flex items-center gap-3">
    <div class="bg-indigo-200 p-2 rounded-md">
      <slot name="icon" size={16} color="indigo" />
    </div>
    <span>
      {name}
    </span>
  </div>
</a>
