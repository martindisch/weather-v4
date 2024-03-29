import adapter from "@sveltejs/adapter-cloudflare";
import { vitePreprocess } from "@sveltejs/vite-plugin-svelte";

/** @type {import('@sveltejs/kit').Config} */
const config = {
  // Consult https://kit.svelte.dev/docs/integrations#preprocessors
  // for more information about preprocessors
  preprocess: vitePreprocess(),
  // suppress warnings on `vite dev` and `vite build`; but even without this, things still work
  onwarn: (warning, handler) => {
    if (warning.code.includes("a11y")) return; // I'm the only user of this app and don't need that
    handler(warning);
  },
  kit: {
    adapter: adapter(),
  },
};

export default config;
