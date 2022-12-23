# weather-app

This SvelteKit app is the client and backend for storing and interacting with
current and historical readings.

## Developing

Start by installing dependencies with `npm install` (or `pnpm install` or
`yarn`).

### Developing with wrangler/Miniflare

Create the local D1 database and run the migration for creating the table.

```bash
npm run migrations
```

Since wrangler serves a built site, it is currently
[not possible](https://github.com/sveltejs/kit/issues/2966)
to get HMR working. Instead you have to rebuild after each change.

```bash
npm run pages
```

### Developing without Miniflare

**Note**: this only works for pages that don't rely on data, so likely none of
them.

```bash
npm run dev

# or start the server and open the app in a new browser tab
npm run dev -- --open
```

## Building

To create a production version of the app:

```bash
npm run build
```

You can preview the production build with `npm run preview`, although this too
won't be super helpful without Miniflare through wrangler. Stick to the section
about wrangler/Miniflare above.
