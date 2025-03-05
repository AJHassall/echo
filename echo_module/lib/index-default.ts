// This module is the CJS entry point for the library.

// The Rust addon.

const addon = require("../native/index.node");

export function start(audio_sources: Array<String>, duration_threshold: number): undefined {
  const message = addon.start(audio_sources, duration_threshold);
}

export function stop(): undefined {
  addon.stop();
}

export function initialise(callback: (eventData: any) => void): undefined {
  addon.initialise(callback);
}

export function get_audio_sources(): Array<String> {
  return addon.get_audio_sources();
}