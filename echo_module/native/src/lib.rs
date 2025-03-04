use audio_recorder::AudioRecorder;
use neon::prelude::*;

mod api;
mod audio_manager;
mod audio_recorder;
mod audio_transcription_controller;
mod event_publisher;
mod jack;
mod transcription_engine;
mod util;
mod web_rtc_vad;

pub fn start(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let duration_threshold = cx.argument::<JsNumber>(1)?;

    let duration_threshold = duration_threshold.value(&mut cx);

    match AudioRecorder::start(duration_threshold) {
        Ok(_) => Ok(cx.undefined()),
        Err(err_msg) => cx.throw_error(err_msg),
    }
}

pub fn stop(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    match AudioRecorder::stop() {
        Ok(_) => Ok(cx.undefined()),
        Err(err_msg) => cx.throw_error(err_msg),
    }
}

pub fn initialise(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let call_back = cx.argument::<JsFunction>(0)?.root(&mut cx);
    let channel = cx.channel();

    match AudioRecorder::initialise(call_back, channel) {
        Ok(_) => Ok(cx.undefined()),
        Err(err_msg) => cx.throw_error(err_msg), // Throw the error to JavaScript
    }
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("start", start)?;
    cx.export_function("stop", stop)?;
    cx.export_function("initialise", initialise)?;

    Ok(())
}
