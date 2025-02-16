use std::io::{self, Write};
use std::sync::mpsc::{self, channel};
use std::thread;

use whisper_rs::{FullParams, SamplingStrategy, WhisperContext, WhisperContextParameters};

pub struct Model {
    ctx: WhisperContext,
    state: whisper_rs::WhisperState,
    initial_prompt: String,
    tokens: Vec<i32>,
    audio_buf: Vec<f32>,
}

impl Model {
    pub fn new(model_path: &str) -> Result<Self, String> {
        let mut context_param = WhisperContextParameters::default();
        context_param.dtw_parameters.mode = whisper_rs::DtwMode::ModelPreset {
            model_preset: whisper_rs::DtwModelPreset::Base,
        };
        context_param.use_gpu = true;

        let ctx = WhisperContext::new_with_params(model_path, context_param)
            .map_err(|e| e.to_string())?;

        let state = ctx.create_state().map_err(|e| e.to_string())?;

        Ok(Model {
            ctx,
            state,
            initial_prompt: String::new(),
            tokens: Vec::new(),
            audio_buf: Vec::new(),
        })
    }

    pub fn transpose_audio(&self, sample: &Vec<f32>) -> Result<String, String> {
        // ... your model inference logic ...

        let mut params = FullParams::new(SamplingStrategy::Greedy { best_of: 0 });
        params.set_n_threads(4);
        params.set_translate(true);
        params.set_language(Some("en"));
        params.set_print_special(false);
        params.set_print_progress(false);
        params.set_print_realtime(false);
        params.set_print_timestamps(false);
        params.set_token_timestamps(true);
        params.set_initial_prompt(&self.initial_prompt);
        params.set_tokens(&self.tokens[..]);



        let num_segments = self
            .state
            .full_n_segments()
            .map_err(|e| e.to_string())?;

        let mut transcript = String::new();

        for i in 0..num_segments {
            if let Ok(token_count) = self.state.full_n_tokens(i) {
                let new_tokens: Vec<i32> = (0..token_count)
                    .map(|j| self.state.full_get_token_id(i, j).map_err(|e| e.to_string()))
                    .collect::<Result<_, String>>()?; // Handle potential errors

                self.tokens.extend(new_tokens); // Add new tokens

                let segment = self
                    .state
                    .full_get_segment_text(i)
                    .map_err(|e| e.to_string())?;

                transcript.push_str(&segment); // Append to transcript

                if self.state.full_get_segment_speaker_turn_next(i) {
                    transcript.push_str("[Speaker Change]");
                }

                self.initial_prompt = segment.clone(); // Update initial prompt
            }
        }

        std::io::stdout().flush().unwrap();
        Ok(transcript)
    }

}