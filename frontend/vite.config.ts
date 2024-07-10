import { defineConfig } from
 
'vite';
import vue from
 
'@vitejs/plugin-vue';

export
 
default defineConfig({
    plugins: [vue()],
    resolve: {
        alias: {
            // Your path shortcuts
            '@': '/src',
            '@components': '/src/components',
            '@utils': '/src/lib/utils', // Adjust paths as needed
            // Add more aliases if desired
        },
    },
});