<script lang="ts">
  import Chart from "$lib/Chart.svelte";
  import Measurements from "$lib/Measurements.svelte";
</script>

<div class="h-full flex flex-col">
  <div class="flex-1 flex flex-col justify-center"><Measurements /></div>
  <div class="flex-1 min-h-[20em]"><Chart /></div>
  <div class="flex-1 min-h-[20em]"><Chart /></div>
</div>
