import { defineConfig } from "vite";
import react, { reactCompilerPreset } from "@vitejs/plugin-react";
import babel from "@rolldown/plugin-babel";
import devtoolsJson from "vite-plugin-devtools-json";

// https://vite.dev/config/
export default defineConfig({
  plugins: [
    devtoolsJson({ uuid: "6ec0bd7f-11c0-43da-975e-2a8ad9ebae0b" }),
    react(),
    babel({ presets: [reactCompilerPreset()] })
  ]
});
