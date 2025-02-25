// This module loads the platform-specific build of the addon on
// the current system. The supported platforms are registered in
// the `platforms` object below, whose entries can be managed by
// by the Neon CLI:
//
//   https://www.npmjs.com/package/@neon-rs/cli


//TODO configure cross platform
module.exports = require('@neon-rs/load').proxy({
  platforms: {
    // 'win32-x64-msvc': () => require('@echo/win32-x64-msvc'),
    // 'darwin-x64': () => require('@echo/darwin-x64'),
    // 'darwin-arm64': () => require('@echo/darwin-arm64'),
    'linux-x64-gnu': () => require('../index.node'),
    // 'linux-arm64-gnu': () => require('@echo/linux-arm64-gnu')
  },
  debug: () => require('../index.node')
});
