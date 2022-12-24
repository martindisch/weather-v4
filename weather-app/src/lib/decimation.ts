import type { Point } from "$lib/types";

export const decimateHourly = (points: Point[]) => {
  if (!points[0]) {
    return [];
  }

  // First encountered date of the current hour bucket we're working on
  let firstHourInstant = points[0].x;
  // Values of the current hour to be averaged and pushed into decimatedData
  let currentBucket: number[] = [];
  // Final decimated data with just one entry for every hour
  const decimated: Point[] = [];

  for (const { x, y } of points) {
    if (x.getHours() !== firstHourInstant.getHours()) {
      // If we encounter a different hour, we've crossed into the next hour and
      // should flush the current one's values by averaging them
      decimated.push({
        x: new Date(
          firstHourInstant.getFullYear(),
          firstHourInstant.getMonth(),
          firstHourInstant.getDay(),
          firstHourInstant.getHours()
        ),
        y:
          currentBucket.reduce((previous, current) => previous + current, 0) / currentBucket.length,
      });

      currentBucket = [];
      firstHourInstant = x;
    }

    currentBucket.push(y);
  }

  // TODO: this is currently discarding the final hour

  return decimated;
};
