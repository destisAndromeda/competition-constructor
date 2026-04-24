import { defineConfig } from 'vitest/config';

export default defineConfig({
  test: {
    fileParallelism: false,
    globals: true,
    sequence: {
      files: 'list',
    },
    include: ['./tests/**/*.ts'],
  },
});
