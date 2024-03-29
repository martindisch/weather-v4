# weather-app

This SvelteKit app is the client and backend for storing and interacting with
current and historical readings. It's deployed on Cloudflare Pages and uses a
D1 database.

## Developing

Start by installing dependencies with `npm install` (or `pnpm install` or
`yarn`).

### With wrangler/Miniflare

Create the local D1 database, run the migration for creating the table and
insert some testdata.

```console
$ npm run migrations
$ npm run testdata
```

Since wrangler serves a built site, it is currently
[not possible](https://github.com/sveltejs/kit/issues/2966)
to get HMR working. Instead you have to rebuild after each change.

```console
$ npm run pages
```

## Building

To create a production version of the app:

```console
$ npm run build
```

## Setting it up in production

1. Create the pages project on Cloudflare with Git integration as per the
   [documentation](https://developers.cloudflare.com/pages/get-started/).
   In the build configuration, make sure to set the root directory to
   `/weather-app` and set the `NODE_VERSION=16` environment variable for
   production and preview environments.
2. [Create a D1 database](https://developers.cloudflare.com/d1/get-started/#3-create-your-database),
   ideally using the name `weather-v3`. Update the values in `wrangler.toml`
   with the output.
3. Initialize the database by running the wrangler command configured in the
   `migrations` script of `package.json`, but without the `--local` option.
4. In the settings of your Pages project
   [configure the binding](https://developers.cloudflare.com/pages/platform/functions/bindings/#d1-databases)
   `DB=weather-v3` for your database. You may want to use a different one for
   the preview environment.
5. Generate an API key (for example with `uuidgen`) and create an encrypted
   environment variable named `API_KEY` holding it for the production
   environment. Again you might want to use a different one for the preview
   environment, as it may be possible to extract the key with a malicious pull
   request.
