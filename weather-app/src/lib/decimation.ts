import type { Point } from "$lib/types";

const bucketSizeMinutes = 15;

export const decimate = (points: Point[]) => {
  if (!points[0]) {
    return [];
  }

  // First encountered date of the current bucket we're working on
  let firstBucketInstant = bucketBase(points[0].x);
  // Values of the current bucket to be averaged and pushed into decimated data
  let currentBucket: number[] = [];
  // Final decimated data with just one entry for every bucket
  const decimated: Point[] = [];

  for (const { x, y } of points) {
    if (bucketBase(x).getTime() !== firstBucketInstant.getTime()) {
      // We've crossed into the next bucket and should flush the current one's
      // values by averaging them
      decimated.push({
        x: firstBucketInstant,
        y:
          currentBucket.reduce((previous, current) => previous + current, 0) / currentBucket.length,
      });
      currentBucket = [];
      firstBucketInstant = bucketBase(x);
    }

    currentBucket.push(y);
  }

  // Note: this is currently discarding the final bucket, but that's fine

  return decimated;
};

const bucketBase = (date: Date) => {
  const bucketDate = new Date(date);
  bucketDate.setMinutes(date.getMinutes() - (date.getMinutes() % bucketSizeMinutes));
  bucketDate.setSeconds(0);
  return bucketDate;
};
