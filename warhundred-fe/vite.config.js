import {defineConfig, loadEnv} from 'vite'
import react from '@vitejs/plugin-react-swc'
import {viteSingleFile} from "vite-plugin-singlefile";

// https://vitejs.dev/config/
export default defineConfig(({mode}) => {
    // Load env file based on `mode` in the current directory.
    // Set the third parameter to '' to load all env regardless of the `VITE_` prefix.
    // eslint-disable-next-line no-undef
    const env = loadEnv(mode, process.cwd(), '')
    const isProduction = mode === 'production'

    return {
        plugins: [react(), viteSingleFile()],
        build: {
            outDir: '../public',
            emptyOutDir: true
        },
        test: {
            globals: true,
            environment: 'jsdom',
            setupFiles: ['./src/test/setup.js'],
            css: true
        },
        define: {
            // Make the server URL available based on the mode
            // In development mode, use VITE_SERVER_URL
            // In production mode, use VITE_PROD_SERVER_URL
            'import.meta.env.VITE_API_URL': isProduction
                ? JSON.stringify(env.VITE_PROD_SERVER_URL)
                : JSON.stringify(env.VITE_SERVER_URL)
        }
    }
})
