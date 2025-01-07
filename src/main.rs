// This example is not going to build in this folder.
// You need to copy this code into your project and add the dependencies whisper_rs and hound in your cargo.toml

use std::io::{self, Write};
use std::sync::mpsc::channel;
use std::thread;
use timer::Timer;
use whisper_rs::{FullParams, SamplingStrategy, WhisperContext, WhisperContextParameters};

mod audio_stream;
mod querybuilder;
mod timer;
mod token_manager;
mod voice_activity_detector;

/// Loads a context and model, processes an audio file, and prints the resulting transcript to stdout.
fn main() {
    // Load a context and model.
    let mut context_param = WhisperContextParameters::default();

    // Enable DTW token level timestamp for known model by using model preset
    context_param.dtw_parameters.mode = whisper_rs::DtwMode::ModelPreset {
        model_preset: whisper_rs::DtwModelPreset::Base,
    };

    let ctx = WhisperContext::new_with_params("whisper-models/ggml-base.bin", context_param)
        .expect("failed to load model");
    // Create a state
    let mut state = ctx.create_state().expect("failed to create key");

    // Create a params object for running the model.
    // The number of past samples to consider defaults to 0.

    let (sender, receiver) = channel();
    thread::spawn(move || {
        audio_stream::setup_callback(sender);
    });

    let mut initial_promt = "".to_string();
    let mut qb = querybuilder::Query::new();

    let mut tokens;
    let mut running = true;

    let mut audio_buf: Vec<f32> = vec![];
    while running {
        let mut params = FullParams::new(SamplingStrategy::Greedy { best_of: 0 });

        // Edit params as needed.
        // Set the number of threads to use to 1.
        params.set_n_threads(4);
        // Enable translation.
        params.set_translate(true);
        // Set the language to translate to to English.
        params.set_language(Some("en"));
        // Disable anything that prints to stdout.
        params.set_print_special(false);
        params.set_print_progress(false);
        params.set_print_realtime(false);
        params.set_print_timestamps(false);
        // Enable token level timestamps
        params.set_token_timestamps(true);
        params.set_initial_prompt("");

        tokens = vec![];

        params.set_initial_prompt(&initial_promt);
        params.set_tokens(&tokens[..]);

        let from_audio_stream = receiver.recv();
        match from_audio_stream {
            Ok(audio) => {

                if audio_buf.len() > 6000 {
                    audio_buf = audio_buf[audio_buf.len()-6000..audio_buf.len()].to_vec();
                }


                let _t = Timer::new();
                audio_buf.extend(audio_stream::resample_to_16000hz(&audio, 48000.0));
                state
                    .full(params.clone(), &audio_buf[..])
                    .expect("failed to run model");
            }
            Err(_) => {
                running = false;
                continue;
            }
        }

        // Iterate through the segments of the transcript.
        let num_segments = state
            .full_n_segments()
            .expect("failed to get number of segments");
        for i in 0..num_segments {
            if let Ok(token_count) = state.full_n_tokens(i) {
                let new_tokens: Vec<i32> = (0..token_count)
                    .map(|j| state.full_get_token_id(i, j).expect("error"))
                    .collect();

                println!("{:?}", new_tokens);

                tokens.extend(new_tokens);

                // Get the transcribed text and timestamps for the current segment.
                let segment = state
                    .full_get_segment_text(i)
                    .expect("failed to get segment");

                if state.full_get_segment_speaker_turn_next(i) {
                    println!("[Speaker Change]")
                }

                qb.extend(&segment);
                qb.parse_questions();
                qb.print_new_questions();

                println!("=====");
                initial_promt = segment.clone();
            };
        }

        io::stdout().flush().unwrap();
    }
}
