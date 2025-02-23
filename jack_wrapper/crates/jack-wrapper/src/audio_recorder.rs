use base64::encode;
use neon::prelude::*;
use reqwest::{Body, Client, Error as ReqwestError};
use serde::Serialize;
use std::time::{Duration, Instant};
use tokio::task::{self, JoinHandle};
use tokio_util::codec::{BytesCodec, FramedRead};

use crate::{
    audio::AudioDataReceiver,
    jack::JackClient,
    vad::{AudioChunkProcessor, VoiceActivityDetector},
};

pub struct AudioRecorder;

impl AudioRecorder {
    #[tokio::main]
    pub async fn initialise(mut cx: FunctionContext) -> JsResult<JsUndefined> {
        println!("Recording started");

        let mut jack_client = JackClient::new();
        let receiver_arc = jack_client.get_audio_receiver();
        jack_client.start_processing();

        let vad = VoiceActivityDetector::new();
        let sample_rate = 16000;
        let mut processor = AudioChunkProcessor::new(vad, sample_rate);

        let start_time = Instant::now();
        let duration = Duration::from_secs(30);

        let mut h = vec![   ];

        while start_time.elapsed() < duration {
            if let Ok(receiver_guard) = receiver_arc.lock() {
                if let Ok(audio_data) = receiver_guard.recv() {
                    processor.process_chunk(audio_data);
                } else {
                    break; // Handle disconnection or other errors
                }
            }

            let new_audio = processor.get_current_audio();
            for data in new_audio.to_vec() {
               h.push(send_f32_base64(data.clone()));
            }

            tokio::time::sleep(Duration::from_millis(10)).await;
        }

        for i in h{
            let s = i.await;
           
        }

        jack_client.stop_processing();
        println!("Recording finished");

        Ok(cx.undefined())
    }
}

// #[derive(Debug, Serialize)]
// struct AudioData {
//     audio: Vec<f32>,
// }

fn send_f32_base64(data: Vec<f32>) -> JoinHandle<Result<(), ReqwestError>> {
    task::spawn(async move {
        let byte_data: Vec<u8> = data.iter().flat_map(|f| f.to_le_bytes()).collect();
        let encoded_data = encode(&byte_data);

        let client = Client::new();
        let response = client
            .post("http://127.0.0.1:8080/transcribe")
            .json(&encoded_data)
            .send()
            .await?
            .error_for_status()?; // Check for HTTP status errors

        println!("API call successful");
        match response.text().await {
            Ok(text) => println!("Response: {}", text),
            Err(e) => {
                eprintln!("Error getting response text: {}", e);
                return Err(ReqwestError::from(e));
            }
        }
        Ok(())
    })
}
#[cfg(test)]
mod tests {
    use super::*;

}
