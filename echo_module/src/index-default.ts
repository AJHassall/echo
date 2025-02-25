// This module is the CJS entry point for the library.

// The Rust addon.

const addon = require('../index.node');

export function start(): undefined {
  const message = addon.start();
}

export function stop(): undefined {
  const message = addon.stop();
}

export function initialise(): undefined {
  const message = addon.initialise();
}

export function get(): Array<String> {
  const message = addon.get();

  return message;
}

export function clear(): undefined{
  const message = addon.clear();
}

