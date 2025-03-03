// echo-transcriber-loader.js
const path = require('path');
const fs = require('fs'); // Import the fs module

module.exports = function (source) {
  const callback = this.async();
  const modulePath = path.resolve(
    __dirname,
    '..',
    'node_modules',
    'echo_module',
    'native', // Ensure this matches your structure
    'index.node'
  );

  // Check if the native module exists using fs.access
  fs.access(modulePath, fs.constants.F_OK, (err) => { // Use fs.constants.F_OK
    if (err) {
      callback(new Error(`Native module not found at: ${modulePath}`));
      return;
    }

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
  });
};