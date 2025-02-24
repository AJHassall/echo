use audio_recorder::AudioRecorder;
use neon::prelude::*;

mod audio_recorder;
mod jack;
mod vad;
mod audio;
mod transcription_engine;
mod api;

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("start", AudioRecorder::start)?;
    cx.export_function("stop", AudioRecorder::stop)?;
    cx.export_function("initialise", AudioRecorder::initialise)?;

    cx.export_function("get", AudioRecorder::get_transcriptions)?;
    cx.export_function("clear", AudioRecorder::clear_transcriptions)?;

    
    Ok(())
}
