import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'

// https://vite.dev/config/
export default defineConfig({
  plugins: [react()],
   define: {
    'process.env': {},
  },
  optimizeDeps: {
    include: ['buffer'],
  },
  server: {
    port: 5173, // порт за замовчуванням
    strictPort: true,
    hmr: true,  // тут увімкнений hot reload
    open: true, // автоматично відкриває браузер
  },
})
