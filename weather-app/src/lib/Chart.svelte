<script lang="ts">
  // @ts-expect-error because the library doesn't have types
  import * as Pancake from "@sveltejs/pancake";
  import { decimate } from "$lib/decimation";
  import type { Point } from "$lib/types";

  export let points: Point[];
  export let unit: string;
  export let color = "black";

  const decimatedPoints = decimate(points);

  const minx = decimatedPoints[0]!.x;
  const maxx = decimatedPoints[decimatedPoints.length - 1]!.x;
  let miny = +Infinity;
  let maxy = -Infinity;

  for (const point of decimatedPoints) {
    if (point.y < miny) {
      miny = point.y;
    }

    if (point.y > maxy) {
      maxy = point.y;
    }
  }
</script>

<div class="chart">
  <Pancake.Chart x1={minx} x2={maxx} y1={miny} y2={maxy}>
    <Pancake.Grid horizontal let:value let:last>
      <div class="grid-line horizontal"><span>{value} {last ? unit : ""}</span></div>
    </Pancake.Grid>

    <Pancake.Svg>
      <Pancake.SvgLine data={decimatedPoints} let:d>
        <path {d} style:stroke={color} />
      </Pancake.SvgLine>
    </Pancake.Svg>

    <Pancake.Quadtree data={decimatedPoints} let:closest>
      {#if closest}
        <Pancake.Point x={closest.x} y={closest.y}>
          <div class="focus" />
          <div class="tooltip">
            <strong>{closest.y.toFixed(1)} {unit}</strong>
            <span>{closest.x.toLocaleTimeString()}</span>
          </div>
        </Pancake.Point>
      {/if}
    </Pancake.Quadtree>
  </Pancake.Chart>
</div>

<style>
  .chart {
    height: 100%;
    padding: 2em 0 2em 2em;
    margin: 0 0 36px 0;
    margin: 0 auto;
  }

  .grid-line {
    position: relative;
    display: block;
  }

  .grid-line.horizontal {
    width: calc(100% + 2em);
    left: -2em;
    border-bottom: 1px dashed #ccc;
  }

  .grid-line span {
    position: absolute;
    left: 0;
    bottom: 2px;
    line-height: 1;
    font-family: sans-serif;
    font-size: 14px;
    color: #999;
  }

  path {
    stroke-linejoin: round;
    stroke-linecap: round;
    stroke-width: 2px;
    fill: none;
  }

  .focus {
    position: absolute;
    width: 10px;
    height: 10px;
    left: -5px;
    top: -5px;
    border: 1px solid black;
    border-radius: 50%;
    box-sizing: border-box;
  }

  .tooltip {
    position: absolute;
    white-space: nowrap;
    bottom: 1em;
    line-height: 1;
    text-shadow:
      0 0 10px white,
      0 0 10px white,
      0 0 10px white,
      0 0 10px white,
      0 0 10px white,
      0 0 10px white,
      0 0 10px white;
  }

  .tooltip strong {
    font-size: 1.4em;
    display: block;
  }
</style>
