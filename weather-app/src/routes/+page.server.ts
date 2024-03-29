import type { PageServerLoad } from "./$types";

export const load: PageServerLoad = async ({ platform }) => {
  const db = platform!.env.DB;

  const { results: measurements } = await db
    .prepare(
      `
        SELECT * FROM measurements
        WHERE timestamp > unixepoch() - 86400
        ORDER BY timestamp ASC;
      `,
    )
    .all();

  return { measurements };
};
