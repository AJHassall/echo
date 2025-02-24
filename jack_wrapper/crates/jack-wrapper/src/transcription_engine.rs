use whisper_rs::{SamplingStrategy, WhisperContext, WhisperContextParameters, WhisperError};

pub struct TranscriptionEngine {  // Renamed for clarity
    state: whisper_rs::WhisperState,
    params: whisper_rs::FullParams<'static, 'static>, // Static lifetimes are okay here
}

impl TranscriptionEngine {
    pub fn new(model_path: &str) -> Result<Self, WhisperError> {

        let mut context_param = WhisperContextParameters::default();

        // Enable DTW token level timestamp for known model by using model preset
        context_param.dtw_parameters.mode = whisper_rs::DtwMode::ModelPreset {
            model_preset: whisper_rs::DtwModelPreset::TinyEn,
        };

        let ctx = WhisperContext::new_with_params(
            model_path,
            context_param,
            
        )?;

        let state = ctx.create_state()?;


        let mut params = whisper_rs::FullParams::new(SamplingStrategy::Greedy { best_of: 1 });
        params.set_n_threads(4);
        params.set_translate(true);
        params.set_language(Some("en"));
        params.set_print_special(false);
        params.set_print_progress(false);
        params.set_print_realtime(false);
        params.set_print_timestamps(false);
        params.set_token_timestamps(true);
        params.set_initial_prompt("");

        Ok(Self {state, params })
    }

    pub fn process_audio(&mut self, audio: &[f32]) -> Result<(), WhisperError> {
        self.state.full(self.params.clone(), audio)?;
        Ok(())
    }

    pub fn get_segments(&self) -> Result<Vec<String>, WhisperError> {
        let num_segments = self.state.full_n_segments()?;
        let mut segments = Vec::new();
        for i in 0..num_segments {
            let segment_text = self.state.full_get_segment_text(i)?;
            segments.push(segment_text);
        }
        Ok(segments)
    }

}