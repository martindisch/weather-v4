<script lang="ts">
  export let title: string;
  export let value: number;
  export let unit: string;
</script>

<div>
  <h1 class="">{title}</h1>
  <span class="text-7xl md:text-8xl">{value}</span><span class="text-4xl md:text-5xl">{unit}</span>
</div>
