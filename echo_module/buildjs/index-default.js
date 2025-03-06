"use strict";
// This module is the CJS entry point for the library.
Object.defineProperty(exports, "__esModule", { value: true });
exports.start = start;
exports.stop = stop;
exports.initialise = initialise;
exports.get_audio_sources = get_audio_sources;
// The Rust addon.
const addon = require("../native/index.node");
function start(audio_sources, duration_threshold) {
    const message = addon.start(audio_sources, duration_threshold);
}
function stop() {
    addon.stop();
}
function initialise(callback) {
    addon.initialise(callback);
}
function get_audio_sources() {
    return addon.get_audio_sources();
}
//# sourceMappingURL=index-default.js.map