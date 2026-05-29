import { sveltekit } from '@sveltejs/kit/vite';
import tailwindcss from '@tailwindcss/vite';
import { defineConfig } from 'vite';

export default defineConfig({
  plugins: [tailwindcss(), sveltekit()],
  server: {
    watch: { usePolling: true },
    host: '0.0.0.0',
    port: 5173,
    allowedHosts: ['super.sjsgroup.site', '43.134.17.13']
  }
});
