import * as path from 'path';



module.exports = require('@neon-rs/load').proxy({
  platforms: {
    'linux-x64-gnu': () => require('/linux-x64-gnu'),
  },
  debug: () => {
    return require('index.node');
  },
});