import { type RequestHandler, json } from "@sveltejs/kit";
import type { Measurement } from "$lib/types";

export const POST: RequestHandler = async ({ request, platform }) => {
  // TODO: some sort of simple authentication

  const db = platform.env.DB;

  const measurements: Measurement[] = await request.json();
  const insertStatement = db.prepare("INSERT INTO Measurements VALUES (?, ?, ?);");
  const insertBatch = measurements.map((measurement) =>
    insertStatement.bind(measurement.timestamp, measurement.temperature, measurement.humidity)
  );

  await db.batch(insertBatch);

  return new Response(null, { status: 201 });
};

export const GET: RequestHandler = async ({ platform }) => {
  const db = platform.env.DB;

  const { results: measurements } = await db
    .prepare(
      `
        SELECT * FROM Measurements
        WHERE Timestamp > unixepoch() - 86400
        ORDER BY Timestamp ASC;
      `
    )
    .all();

  return json(measurements);
};
