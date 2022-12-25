import { type RequestHandler, error } from "@sveltejs/kit";
import type { Measurement } from "$lib/types";

export const POST: RequestHandler = async ({ request, platform }) => {
  verifyApiKey(request.headers.get("x-api-key"), platform.env.API_KEY);

  const db = platform.env.DB;

  const measurements: Measurement[] = await request.json();
  const insertStatement = db.prepare("INSERT INTO measurements VALUES (?, ?, ?);");
  const insertBatch = measurements.map((measurement) =>
    insertStatement.bind(measurement.timestamp, measurement.temperature, measurement.humidity)
  );

  await db.batch(insertBatch);

  return new Response(null, { status: 201 });
};

// Because this route is the only place we need the following code, it hasn't
// been turned into a middleware yet
const verifyApiKey = (headerKey: string | null, environmentKey: string) => {
  if (headerKey !== environmentKey) {
    throw error(401, "Invalid API key");
  }
};
