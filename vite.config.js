import { defineConfig } from 'vite'
import { svelte } from '@sveltejs/vite-plugin-svelte'
import windicss from 'vite-plugin-windicss'
import ViteRsw from 'vite-plugin-rsw'

export default defineConfig({
  plugins: [
    svelte(),
    ViteRsw.default({
      cli: 'pnpm',
      root: './crates',
      crates: ['hello'],
    }),
    windicss.default(),
  ],
})
