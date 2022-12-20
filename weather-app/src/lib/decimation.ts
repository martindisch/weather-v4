interface Value {
  date: Date;
  value: number;
}

export const decimateHourly = (values: Value[]) => {
  if (!values[0]) {
    return [];
  }

  // First encountered date of the current hour bucket we're working on
  let firstHourInstant = values[0].date;
  // Values of the current hour to be averaged and pushed into decimatedData
  let currentBucket: number[] = [];
  // Final decimated data with just one entry for every hour
  const decimated: Value[] = [];

  for (const { date, value } of values) {
    if (date.getHours() !== firstHourInstant.getHours()) {
      // If we encounter a different hour, we've crossed into the next hour and
      // should flush the current one's values by averaging them
      decimated.push({
        date: new Date(
          firstHourInstant.getFullYear(),
          firstHourInstant.getMonth(),
          firstHourInstant.getDay(),
          firstHourInstant.getHours()
        ),
        value:
          currentBucket.reduce((previous, current) => previous + current, 0) / currentBucket.length,
      });

      currentBucket = [];
      firstHourInstant = date;
    }

    currentBucket.push(value);
  }

  // TODO: this is currently discarding the final hour

  return decimated;
};
