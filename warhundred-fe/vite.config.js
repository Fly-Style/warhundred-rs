import {defineConfig} from 'vite'
import react from '@vitejs/plugin-react-swc'
import {viteSingleFile} from "vite-plugin-singlefile";

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [react(), viteSingleFile()],
  build: {
    outDir: '../public',
    emptyOutDir : true
  },
  test: {
    globals: true,
    environment: 'jsdom',
    setupFiles: ['./src/test/setup.js'],
    css: true
  }
})
