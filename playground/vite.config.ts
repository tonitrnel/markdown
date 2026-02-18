import { defineConfig, searchForWorkspaceRoot } from 'vite';
import solid from 'vite-plugin-solid';
import wasm from 'vite-plugin-wasm';
import topLevelAwait from 'vite-plugin-top-level-await';
import tailwindcss from '@tailwindcss/vite';

export default defineConfig({
  server: {
    port: 12245,
    fs: {
      allow: [searchForWorkspaceRoot(process.cwd()), '../wasm-binding/pkg/'],
    },
  },
  build: {
    target: 'esnext',
  },
  plugins: [wasm(), topLevelAwait(), solid(), tailwindcss()],
});
