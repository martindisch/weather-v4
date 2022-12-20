<script lang="ts">
  import { browser } from "$app/environment";
  import csv from "./data.js";
  import { Chart, registerables } from "chart.js";
  import { onMount } from "svelte";
  import { DateTime } from "luxon";
  import "chartjs-adapter-luxon";

  Chart.register(...registerables);

  let lineChartElement: HTMLCanvasElement;

  const points = csv.split("\n").map((str) => {
    let [timestamp, temperature, humidity] = str.split(",").map(parseFloat);
    return { date: DateTime.fromMillis(timestamp * 1000), temperature, humidity };
  });

  let data = {
    labels: points.map((p) => p.date),
    datasets: [
      {
        label: "Temperature",
        data: points.map((p) => {
          return { x: p.date, y: p.temperature };
        }),
        borderColor: "red",
        backgroundColor: "red",
        yAxisID: "y",
        tension: 0.2,
      },
      {
        label: "Humidity",
        data: points.map((p) => {
          return { x: p.date, y: p.humidity };
        }),
        borderColor: "blue",
        backgroundColor: "blue",
        yAxisID: "y1",
      },
    ],
  };

  onMount(() => {
    if (browser) {
      new Chart(lineChartElement, {
        type: "line",
        data,
        options: {
          parsing: false,
          responsive: true,
          interaction: {
            mode: "index",
            intersect: false,
          },
          plugins: {
            title: {
              display: true,
              text: "Combined chart",
            },
            decimation: {
              enabled: true,
              algorithm: "lttb",
            },
          },
          scales: {
            x: {
              type: "time",
              time: {
                tooltipFormat: "DD T",
              },
            },
            y: {
              type: "linear",
              display: true,
              position: "left",
            },
            y1: {
              type: "linear",
              display: true,
              position: "right",
              grid: {
                drawOnChartArea: false,
              },
            },
          },
        },
      });
    }
  });
</script>

<div>
  <canvas bind:this={lineChartElement} />
</div>
