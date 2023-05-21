import { defineConfig } from "vite";
import { readFileSync } from "fs";

export default defineConfig({
  server: {
    host: true,
    watch: {
      usePolling: true,
    },
    https: !process.env.CI && {
      cert: readFileSync("./localhost.pem"),
      key: readFileSync("./localhost-key.pem"),
    },
  },
});
