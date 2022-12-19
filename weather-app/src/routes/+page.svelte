<script lang="ts">
  import Chart from "$lib/Chart.svelte";
  import Measurement from "$lib/Measurement.svelte";
</script>

<div class="flex justify-center gap-10">
  <Measurement title="Temperature" value={21} unit="°C" />
  <Measurement title="Relative Humidity" value={60} unit="%" />
</div>
<Chart />
