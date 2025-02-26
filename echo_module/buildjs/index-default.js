"use strict";
// This module is the CJS entry point for the library.
Object.defineProperty(exports, "__esModule", { value: true });
exports.start = start;
exports.stop = stop;
exports.initialise = initialise;
exports.get = get;
exports.clear = clear;
// The Rust addon.
const addon = require('../native/index.node');
function start() {
    const message = addon.start();
}
function stop() {
    const message = addon.stop();
}
function initialise() {
    const message = addon.initialise();
}
function get() {
    const message = addon.get();
    return message;
}
function clear() {
    const message = addon.clear();
}
//# sourceMappingURL=index-default.js.map