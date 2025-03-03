import { defineConfig } from 'vite';
import path from 'path';
// Import the plugins you created
import { nodejsPolarsNativeFilePlugin } from './plugins/nodejs-polars-native-file-plugin.js'; // Adjust path if needed
import { nodejsPolarsDirnamePlugin } from './plugins/nodejs-polars-dirname-plugin.js'; // Adjust path if needed

import nodeResolve from '@rollup/plugin-node-resolve';

const projectRootDir = path.resolve(__dirname);
const nativeDir = path.resolve(projectRootDir, './node_modules/echo_module/native'); // Path to your native addon directory
const outDir = path.resolve(projectRootDir, './dist'); // Or your Vite output dir, adjust if needed
const nodeFiles = [path.resolve(nativeDir, 'index.node')]; // Array of .node files to handle


// Dummy logger (replace with your actual logger if you have one)
const logger = {
    debug: (pluginName: any, message: any, ...args: any) => console.debug(`[${pluginName}] ${message}`, ...args),
    error: (pluginName: any, message: any, ...args: any) => console.error(`[${pluginName}] ERROR: ${message}`, ...args),
};


export default defineConfig({
    plugins: [

        nodeResolve({ // Important for resolving modules, keep this
            extensions: ['.js', '.json', '.node'], // Add .node extension to be resolved
        }),
        nodejsPolarsDirnamePlugin(logger), // Add the dirname plugin
        nodejsPolarsNativeFilePlugin(logger, nodeFiles, outDir), // Add the native file plugin
    ],


    build: {
        rollupOptions: {
            input: path.join(projectRootDir, 'src/index.js'), // Or your renderer entry point
            output: {
                assetFileNames: (assetInfo) => {
                    if (assetInfo.name && assetInfo.name.endsWith('.node')) {
                        return 'assets/[name][extname]';
                    }
                    return 'assets/[name]-[hash][extname]';
                },
            },
            // external: ['.node'],  <-- Remove or comment out this 'external' line, the plugin handles it
        },
    },
});