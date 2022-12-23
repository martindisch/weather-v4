import { type RequestHandler, json } from "@sveltejs/kit";

export const POST: RequestHandler = async ({ request, platform }) => {
  const db = platform.env.DB;

  const { results } = await db.prepare("SELECT * FROM Measurements").all();

  return json(results);
};
