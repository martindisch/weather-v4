<script lang="ts">
  export let title: string;
  export let value: number;
  export let unit: string;
</script>

<div>
  <h1 class="">{title}</h1>
  <span class="text-8xl md:text-9xl">{value}</span><span class="text-5xl md:text-6xl">{unit}</span>
</div>
