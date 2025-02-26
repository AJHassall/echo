// echo-transcriber-loader.js
const path = require('path');

module.exports = function (source) {
  const callback = this.async();
  const modulePath = path.resolve(
    __dirname,
    '..',
    'node_modules',
    'echo_transcriber',
    'native',
    'index.node'
  );

console.log("inside loaderer");


    // Attempt to load the native module using __non_webpack_require__
    const loadModule = `
      try {
        module.exports = __non_webpack_require__('${modulePath}');
      } catch (error) {
        console.error('Error loading echo_transcriber native module:', error);
        module.exports = {}; // Or handle the error appropriately
      }
    `;

    callback(null, loadModule);
  
};