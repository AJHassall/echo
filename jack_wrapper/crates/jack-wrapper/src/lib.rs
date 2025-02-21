use audio_recorder::AudioRecorder;
use neon::prelude::*;
mod audio_recorder;
mod jack;

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("initialise", AudioRecorder::initialise)?;
    
    Ok(())
}
