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
function start(silence_threshold, duration_threshhold) {
    const message = addon.start(silence_threshold, duration_threshhold);
}
function stop() {
    addon.stop();
}
function initialise(callback) {
    addon.initialise(callback);
}
function get() {
    const transcriptions = addon.get();
    return transcriptions;
}
function clear() {
    addon.clear();
}
function get_energy() {
    return addon.get_energy();
}
//# sourceMappingURL=index-default.js.map