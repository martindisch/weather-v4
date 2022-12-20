<script>
  import * as Pancake from "@sveltejs/pancake";
  import csv from "./data.js";

  const points = csv.split("\n").map((str) => {
    let [timestamp, temperature, humidity] = str.split(",").map(parseFloat);
    return { date: new Date(timestamp * 1000), temperature, humidity };
  });

  let minx = points[0].date;
  let maxx = points[points.length - 1].date;
  let miny = +Infinity;
  let maxy = -Infinity;

  for (let i = 0; i < points.length; i += 1) {
    const point = points[i];

    if (point.temperature < miny) {
      miny = point.temperature;
    }

    if (point.temperature > maxy) {
      maxy = point.temperature;
    }
  }

  const pc = (date) => {
    return (100 * (date - minx)) / (maxx - minx);
  };
</script>

<div class="chart">
  <Pancake.Chart x1={minx} x2={maxx} y1={miny} y2={maxy}>
    <Pancake.Grid horizontal count={5} let:value let:last>
      <div class="grid-line horizontal"><span>{value} {last ? "°C" : ""}</span></div>
    </Pancake.Grid>

    <Pancake.Grid vertical count={5} let:value>
      <div class="grid-line vertical" />
      <span class="year-label">{value}</span>
    </Pancake.Grid>

    <Pancake.Svg>
      <Pancake.SvgLine data={points} x={(d) => d.date} y={(d) => d.temperature} let:d>
        <path class="trend" {d} />
      </Pancake.SvgLine>
    </Pancake.Svg>

    <Pancake.Quadtree data={points} x={(d) => d.date} y={(d) => d.temperature} let:closest>
      {#if closest}
        <Pancake.Point x={closest.date} y={closest.temperature} let:d>
          <div class="focus" />
          <div class="tooltip" style="transform: translate(-{pc(closest.date)}%,0)">
            <strong>{closest.temperature} °C</strong>
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
