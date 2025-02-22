use std::sync::{mpsc, Arc, Mutex};

pub trait AudioDataReceiver {
    fn get_audio_receiver(&self) -> Arc<Mutex<mpsc::Receiver<Vec<f32>>>>;
}

