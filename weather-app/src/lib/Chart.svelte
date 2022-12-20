<script lang="ts">
  import * as Pancake from "@sveltejs/pancake";
  import csv from "./data.js";
  import { decimateHourly } from "$lib/decimation";

  const points = csv.split("\n").map((str) => {
    let [timestamp, temperature, humidity] = str.split(",").map(parseFloat);
    return { date: new Date(<number>timestamp * 1000), temperature, humidity };
  });

  const decimatedTemperature = decimateHourly(
    points.map((point) => ({ date: point.date, value: <number>point.temperature }))
  );

  let minx = decimatedTemperature[0]!.date;
  let maxx = decimatedTemperature[decimatedTemperature.length - 1]!.date;
  let miny = +Infinity;
  let maxy = -Infinity;

  for (let i = 0; i < decimatedTemperature.length; i += 1) {
    const point = decimatedTemperature[i];

    if (point.value < miny) {
      miny = point.value;
    }

    if (point.value > maxy) {
      maxy = point.value;
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
      <span class="year-label">{new Date(value).toLocaleTimeString()}</span>
    </Pancake.Grid>

    <Pancake.Svg>
      <Pancake.SvgLine data={decimatedTemperature} x={(d) => d.date} y={(d) => d.value} let:d>
        <path class="trend" {d} />
      </Pancake.SvgLine>
    </Pancake.Svg>

    <Pancake.Quadtree data={decimatedTemperature} x={(d) => d.date} y={(d) => d.value} let:closest>
      {#if closest}
        <Pancake.Point x={closest.date} y={closest.value}>
          <div class="focus" />
          <div class="tooltip">
            <strong>{closest.value} °C</strong>
            <span>{closest.date}</span>
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

  .year-label {
    position: absolute;
    width: 4em;
    left: -2em;
    bottom: -30px;
    font-family: sans-serif;
    font-size: 14px;
    color: #999;
    text-align: center;
  }

  path.trend {
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
    /* background-color: white; */
    line-height: 1;
    text-shadow: 0 0 10px white, 0 0 10px white, 0 0 10px white, 0 0 10px white, 0 0 10px white,
      0 0 10px white, 0 0 10px white;
  }

  .tooltip strong {
    font-size: 1.4em;
    display: block;
  }
</style>
