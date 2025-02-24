use whisper_rs::{DtwParameters, SamplingStrategy, WhisperContext, WhisperContextParameters};

pub struct TranscriptionEngine {  // Renamed for clarity
    ctx: WhisperContext,
    state: whisper_rs::WhisperState,
    params: whisper_rs::FullParams<'static, 'static>, // Static lifetimes are okay here
}

impl TranscriptionEngine {
    pub fn new(model_path: &str) -> Result<Self, String> {

        let mut context_param = WhisperContextParameters::default();

        // Enable DTW token level timestamp for known model by using model preset
        context_param.dtw_parameters.mode = whisper_rs::DtwMode::ModelPreset {
            model_preset: whisper_rs::DtwModelPreset::TinyEn,
        };

        let ctx = WhisperContext::new_with_params(
            model_path,
            context_param,
            
        )
        .map_err(|e| e.to_string())?;
    
        let state = ctx.create_state().map_err(|e| e.to_string())?;


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

        Ok(Self { ctx, state, params })
    }

    pub fn process_audio(&mut self, audio: &[f32]) -> Result<(), String> {
        self.state.full(self.params.clone(), audio).map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn get_segments(&self) -> Result<Vec<String>, String> {
        let num_segments = self.state.full_n_segments().map_err(|e| e.to_string())?;
        let mut segments = Vec::new();
        for i in 0..num_segments {
            let segment_text = self.state.full_get_segment_text(i).map_err(|e| e.to_string())?;
            segments.push(segment_text);
        }
        Ok(segments)
    }

    pub fn get_tokens(&self, segment_index: usize) -> Result<Vec<i32>, String> {
        let token_count = self.state.full_n_tokens(segment_index.try_into().unwrap()).map_err(|e| e.to_string())?;
        let mut tokens = Vec::new();
        for j in 0..token_count {
            let token_id = self.state.full_get_token_id(segment_index.try_into().unwrap(), j).map_err(|e| e.to_string())?;
            tokens.push(token_id);
        }
        Ok(tokens)
    }

    // ... any other methods you need ...
}