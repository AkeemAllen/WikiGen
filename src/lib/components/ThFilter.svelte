<script lang="ts">
  import type { DataHandler } from "@vincjo/datatables";
  export let handler: DataHandler;
  export let filterBy: string;
  let value: string;
</script>

<th>
  <input
    class="block w-full pl-2 rounded-md border-0 py-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6 disabled:bg-gray-100 disabled:text-gray-400"
    type="text"
    placeholder="Filter"
    bind:value
    on:input={() => {
      if (filterBy) handler.filter(value, filterBy);
    }}
  />
</th>
