<script lang="ts">
  import Chart from "$lib/Chart.svelte";
  import Measurements from "$lib/Measurements.svelte";
  import type { Measurement, Point } from "$lib/types";

  export let data: { measurements: Measurement[] };

  const { temperaturePoints, humidityPoints } = data.measurements.reduce(
    (previous, current) => {
      const date = new Date(current.timestamp * 1000);
      previous.temperaturePoints.push({ x: date, y: current.temperature });
      previous.humidityPoints.push({ x: date, y: current.humidity });

      return previous;
    },
    { temperaturePoints: <Point[]>[], humidityPoints: <Point[]>[] }
  );
</script>

<svelte:head>
  <title>Weather</title>
</svelte:head>

<div class="h-full flex flex-col">
  <div class="flex-1 flex flex-col justify-center"><Measurements /></div>
  <div class="flex-1 min-h-[20em]"><Chart points={temperaturePoints} /></div>
  <div class="flex-1 min-h-[20em]"><Chart points={humidityPoints} /></div>
</div>
