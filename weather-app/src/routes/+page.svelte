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
  <div class="flex-1 basis-1/6 flex flex-col justify-center"><Measurements /></div>
  <div class="flex-1 basis-1/3 min-h-[15em]">
    <Chart points={temperaturePoints} unit="°C" color="red" />
  </div>
  <div class="flex-1 basis-1/3 min-h-[15em]">
    <Chart points={humidityPoints} unit="%" color="blue" />
  </div>
  <div class="h-[5vh]" />
</div>
