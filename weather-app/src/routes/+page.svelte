<script lang="ts">
  import Chart from "$lib/Chart.svelte";
  import Measurement from "$lib/Measurement.svelte";
</script>

<div class="h-full flex flex-col">
  <div class="flex-1">
    <div class="flex justify-center gap-10">
      <Measurement title="Temperature" value={21} unit="°C" />
      <Measurement title="Relative Humidity" value={60} unit="%" />
    </div>
  </div>
  <div class="flex-1 min-h-[20em]"><Chart /></div>
  <div class="flex-1 min-h-[20em]"><Chart /></div>
</div>
