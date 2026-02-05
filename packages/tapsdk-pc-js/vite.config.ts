import { defineConfig } from 'vite';
import dts from 'vite-plugin-dts';
import { resolve } from 'path';

export default defineConfig({
  build: {
    lib: {
      entry: resolve(__dirname, 'ts/index.ts'),
      name: 'TapSdkPc',
      formats: ['es', 'cjs'],
      fileName: (format) => `index.${format === 'es' ? 'js' : 'cjs'}`,
    },
    rollupOptions: {
      // Externalize Node.js built-ins and native addon
      external: [
        /\.node$/,
        'module',
        'path',
        'url',
        'fs',
      ],
      output: {
        exports: 'named',
      },
    },
    sourcemap: true,
    minify: false,
    // Target Node.js
    target: 'node18',
  },
  plugins: [
    dts({
      include: ['ts/**/*'],
      exclude: ['ts/**/*.test.ts'],
      outDir: 'dist',
      rollupTypes: true,
    }),
  ],
});
