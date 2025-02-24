// This module is the CJS entry point for the library.

// The Rust addon.
import * as addon from './load.cjs';

// Use this declaration to assign types to the addon's exports,
// which otherwise by default are `any`.
declare module "./load.cjs" {
  function start(): undefined;
  function stop(): undefined;
  function initialise(): undefined;
  function clear(): undefined;

  function get(): Array<String>;
}


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

