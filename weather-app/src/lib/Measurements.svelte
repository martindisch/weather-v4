<script lang="ts">
  import Measurement from "$lib/Measurement.svelte";

  export let temperature: number;
  export let humidity: number;
</script>

<div class="flex justify-center gap-4 md:gap-8">
  <Measurement title="Temperature" value={temperature} unit="°C" />
  <Measurement title="Relative Humidity" value={humidity} unit="%" />
</div>
