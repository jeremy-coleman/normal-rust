import reactRefresh from '@vitejs/plugin-react-refresh'
import { defineConfig } from 'vite'
import viteRsw from './tools/vite-plugin-rsw'

export default defineConfig({
  plugins: [
    reactRefresh(),
    viteRsw({
      root: "src/crates",
      crates: ['compute-normals'],
    }),
  ],
  server: {
    port: 1234,
  },
  // esbuild: {
  //   jsxInject: "import React from 'react'",
  // },
})
