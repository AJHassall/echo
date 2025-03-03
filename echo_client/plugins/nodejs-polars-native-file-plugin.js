import path from 'path';
import fs from 'fs';

const pluginName = 'nodejs-polars-native-file-plugin'

/**
 * @param {Logger} logger
 * @param {string[]} nodeFiles
 * @param {string} outDir
 * @returns {import('rollup').Plugin}
 */
export const nodejsPolarsNativeFilePlugin = (logger, nodeFiles, outDir) => {
  return {
    name: pluginName,
    resolveId(id) {
      if (nodeFiles.find(file => path.basename(id) === path.basename(file))) {
        return id;
      }
      return null;
    },
    transform(code, id) {
      const file = nodeFiles.find(file => path.basename(id) === path.basename(file))
      if (file) {
        logger.debug(name, 'transform', id)
        // https://stackoverflow.com/questions/66378682/nodejs-loading-es-modules-and-native-addons-in-the-same-project
        // this makes the .node file load at runtime from an esm context. .node files aren't native to esm, so we have to create a custom require function to load them. The custom require function is equivalent to the require function in commonjs, thus allowing the .node file to be loaded.
        return `
            // create a custom require function to load .node files
            import { createRequire } from 'module';
            const customRequire = createRequire(import.meta.url)

            // load the .node file expecting it to be in the same directory as the output bundle
            const content = customRequire('./${file}')

            // export the content straight back out again
            export default content
            `
      }
      return null
    },
    generateBundle: async () => {
      for (const fileAbs of nodeFiles) {
        const file = path.basename(fileAbs)
        // copy the .node file to the output directory
        const out = `${outDir}/${file}`
        const src = `${fileAbs}`
        logger.debug(name, 'copy', src, 'to', out)
        const nodeFile = fs.readFileSync(src)
        fs.mkdirSync(path.dirname(out), { recursive: true })
        fs.writeFileSync(out, nodeFile)
      }
    },
  }
}