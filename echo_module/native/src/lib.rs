use audio_recorder::AudioRecorder;
use futures::channel;
use neon::prelude::*;

mod audio_recorder;
mod jack;
mod audio;
mod transcription_engine;
mod api;
mod event_publisher;
mod web_rtc_vad;
mod audio_manager;
mod audio_transcription_controller;

pub fn start(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let silence_threshhold = cx.argument::<JsNumber>(0)?;
    let duration_threshold = cx.argument::<JsNumber>(1)?;

    let silence_threshhold = silence_threshhold.value(&mut cx);
    let duration_threshold = duration_threshold.value(&mut cx);

    AudioRecorder::start(silence_threshhold, duration_threshold);

    Ok(cx.undefined())    
}

pub fn stop(mut cx: FunctionContext) -> JsResult<JsUndefined> {

    AudioRecorder::stop();

    Ok(cx.undefined())    
}

pub fn initialise(mut cx: FunctionContext) -> JsResult<JsUndefined>{
    let call_back = cx.argument::<JsFunction>(0)?.root(&mut cx);
    let channel = cx.channel();

    AudioRecorder::initialise(call_back, channel);

    Ok(cx.undefined())    
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("start", start)?;
    cx.export_function("stop", stop)?;
    cx.export_function("initialise", initialise)?;
    
    Ok(())
}
