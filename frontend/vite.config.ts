import { defineConfig } from 'vite'
import { svelte } from '@sveltejs/vite-plugin-svelte'

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [svelte()],
  build: {
        manifest: true,
        emptyOutDir: true,
        outDir: '../build/public'
  }
})
