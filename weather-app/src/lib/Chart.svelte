<script lang="ts">
  // @ts-ignore
  import * as Pancake from "@sveltejs/pancake";
  import { decimateHourly } from "$lib/decimation";
  import csv from "$lib/data.js";

  const data = csv.split("\n").map((str) => {
    const [timestamp, temperature, humidity] = str.split(",").map(parseFloat);
    return { date: new Date(<number>timestamp * 1000), temperature, humidity };
  });

  const points = decimateHourly(
    data.map((point) => ({ x: point.date, y: <number>point.temperature }))
  );

  const minx = points[0]!.x;
  const maxx = points[points.length - 1]!.x;
  let miny = +Infinity;
  let maxy = -Infinity;

  for (const point of points) {
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
      <div class="grid-line horizontal"><span>{value} {last ? "°C" : ""}</span></div>
    </Pancake.Grid>

    <Pancake.Grid vertical count={8} let:value>
      <div class="grid-line vertical" />
      <span class="label">{new Date(value).toLocaleTimeString()}</span>
    </Pancake.Grid>

    <Pancake.Svg>
      <Pancake.SvgLine data={points} let:d>
        <path {d} />
      </Pancake.SvgLine>
    </Pancake.Svg>

    <Pancake.Quadtree data={points} let:closest>
      {#if closest}
        <Pancake.Point x={closest.x} y={closest.y}>
          <div class="focus" />
          <div class="tooltip">
            <strong>{closest.y.toFixed(1)} °C</strong>
            <span>{closest.x.toLocaleTimeString()}</span>
          </div>
        </Pancake.Point>
      {/if}
    </Pancake.Quadtree>
  </Pancake.Chart>
</div>

<style>
  .chart {
    height: 650px;
    padding: 3em 0 2em 2em;
    margin: 0 0 36px 0;
    max-width: 80em;
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

  .grid-line.vertical {
    height: 100%;
    border-left: 1px dashed #ccc;
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

  .label {
    position: absolute;
    width: 4em;
    left: -2em;
    bottom: -30px;
    font-family: sans-serif;
    font-size: 14px;
    color: #999;
    text-align: center;
  }

  path {
    stroke: #ff3e00;
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
    width: 8em;
    bottom: 1em;
    line-height: 1;
    text-shadow: 0 0 10px white, 0 0 10px white, 0 0 10px white, 0 0 10px white, 0 0 10px white,
      0 0 10px white, 0 0 10px white;
  }

  .tooltip strong {
    font-size: 1.4em;
    display: block;
  }
</style>
