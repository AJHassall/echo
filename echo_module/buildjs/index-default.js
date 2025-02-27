"use strict";
// This module is the CJS entry point for the library.
Object.defineProperty(exports, "__esModule", { value: true });
exports.start = start;
exports.stop = stop;
exports.initialise = initialise;
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
//# sourceMappingURL=index-default.js.map