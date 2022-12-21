# weather-app

This SvelteKit app is the client and backend for storing and interacting with
current and historical readings.

## Developing

Start by installing dependencies with `npm install` (or `pnpm install` or
`yarn`).

### Developing with wrangler/Miniflare

Create the local D1 database and run the migration for creating the table.

```console
$ npm run migrations
```

Since wrangler serves a built site, it is currently
[not possible](https://github.com/sveltejs/kit/issues/2966)
to get HMR working. Instead you have to rebuild after each change.

```console
$ npm run pages
```

### Developing without Miniflare

**Note**: this only works for pages that don't rely on data, so likely none of
them. It can still be useful for a fast feedback cycle when doing some pixel
pushing.

```console
$ npm run dev

# or start the server and open the app in a new browser tab
$ npm run dev -- --open
```

## Building

To create a production version of the app:

```console
$ npm run build
```

You can preview the production build with `npm run preview`, although this too
won't be super helpful without Miniflare through wrangler. Stick to the section
about wrangler/Miniflare above.

## Setting it up in production

1. Create the pages project on Cloudflare with Git integration as per the
   [documentation](https://developers.cloudflare.com/pages/get-started/).
   In the build configuration, make sure to set the root directory to
   `/weather-app` and set the `NODE_VERSION=16` environment variable for
   production and preview environments.
2. [Create a D1 database](https://developers.cloudflare.com/d1/get-started/#3-create-your-database)
   ideally using the name `weather-v3`.
3. Populate the database by running the command configured in the `migrations`
   script of `package.json`, but without the `--local` option.
4. In the settings of your Pages project
   [configure the bindings](https://developers.cloudflare.com/pages/platform/functions/bindings/#d1-databases)
   for your database in both environments.
