use webrtc_vad::{SampleRate, Vad, VadMode};

/// A simplified facade for the webrtc_vad crate.
pub struct WebRtcVadFacade {
    vad: Vad,
    sample_rate: SampleRate,
}

impl WebRtcVadFacade {
    pub fn new(sample_rate: i32, mode: VadMode) -> Result<Self, String> {

        let sample_rate_enum = SampleRate::try_from(sample_rate)?;

        let mut vad = Vad::new();
        vad.set_sample_rate(sample_rate_enum);        
        vad.set_mode(mode);

        let sample_rate_again = SampleRate::try_from(sample_rate)?;
        Ok(Self { vad, sample_rate: sample_rate_again })
    }

    
    pub fn is_speech(&mut self, frame: &[f32]) -> Result<bool, String> {
        let expected_frame_size = self.calculate_expected_frame_size();
        if frame.len() != expected_frame_size {
            return Err(format!(
                "Invalid frame size. Expected {}, got {}",
                expected_frame_size,
                frame.len()
            ));
        }

        match self.vad.is_voice_segment(&WebRtcVadFacade::f32_to_i16(frame)) {
            Ok(is_speech) => Ok(is_speech),
            Err(()) => Err("Invalid frame length".to_string()),
        }
    }

    pub fn f32_to_i16(input: &[f32]) -> Vec<i16> {
        let mut output: Vec<i16> = Vec::with_capacity(input.len());
        for sample in input {
            // Scale f32 sample to i16 range
            let scaled_sample = sample * 32767.0; // Or 32768.0 for symmetrical range
    
            // Clip/Saturate to ensure it's within i16 range
            let clipped_sample = scaled_sample.clamp(-32768.0,32767.0);
    
            // Cast to i16 (truncates towards zero, which is usually fine for audio)
            let i16_sample = clipped_sample as i16;
            output.push(i16_sample);
        }
        output
    }

    /// Calculates the expected frame size based on the sample rate and frame duration (30ms).
    pub fn calculate_expected_frame_size(&self) -> usize {
        (48000.0 * 0.03) as usize // 30ms frame.
    }

    pub fn set_mode(&mut self, mode: VadMode) {
        self.vad.set_mode(mode);
    }

}

#[cfg(test)]
mod tests {
    

}