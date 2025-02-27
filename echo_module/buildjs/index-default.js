"use strict";
// This module is the CJS entry point for the library.
Object.defineProperty(exports, "__esModule", { value: true });
exports.start = start;
exports.stop = stop;
exports.initialise = initialise;
exports.get = get;
exports.clear = clear;
exports.get_energy = get_energy;
// The Rust addon.
const addon = require('../native/index.node');
function start(silence_threshold, duration_threshhold, callback) {
    const message = addon.start(silence_threshold, duration_threshhold, callback);
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
function get_energy() {
    return addon.get_energy();
}
//# sourceMappingURL=index-default.js.map