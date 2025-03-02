// This module is the CJS entry point for the library.

// The Rust addon.

const addon = require('../native/index.node');

export function start(silence_threshold: number, duration_threshhold: number): undefined {
  const message =addon.start(silence_threshold, duration_threshhold);
}

export function stop(): undefined {
  addon.stop();
}

export function initialise(callback: () => object): undefined {
  addon.initialise(callback);
}


