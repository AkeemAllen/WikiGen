<script lang="ts">
  export let name = "";
  export let route = "";
</script>

<a
  href={route}
  id="sidebar-left"
  class="btn border-0 hover:cursor-pointer rounded-md p-2 bg-blue-200 w-40"
>
  {name}
</a>
