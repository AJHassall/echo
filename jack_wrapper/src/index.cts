// This module is the CJS entry point for the library.

// The Rust addon.
import * as addon from './load.cjs';

// Use this declaration to assign types to the addon's exports,
// which otherwise by default are `any`.
declare module "./load.cjs" {
  function Start(): string;
  function Stop(): string;
}

export type Greeting = {
  message: string
};


export function start(): undefined {
  const message = addon.Start();

}

export function stop() : undefined {
  const message = addon.Stop();
}
