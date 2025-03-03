const pluginName = 'nodejs-polars-dirname-plugin'

/**
 * @param {Logger} logger
 * @returns {import('rollup').Plugin}
 */
export const nodejsPolarsDirnamePlugin = (logger) => {
  return {
    name: pluginName,
    transform(code, id) {
      logger.debug(pluginName, 'transform', id)
      // replace all occurrences of __dirname with import.meta.url
      const transformedCode = code.replace(/__dirname/g, 'import.meta.url')
      return {
        code: transformedCode,
        map: null,
      };
    },
  };
};