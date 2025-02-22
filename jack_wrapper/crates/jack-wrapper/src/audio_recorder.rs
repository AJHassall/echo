use std::{path::Path, thread, time::{Duration, Instant}};

use neon::{handle, prelude::*};
use tokio::{runtime::Runtime, sync::mpsc, time::error::Error};

use crate::{audio::AudioDataReceiver, jack::JackClient, vad::{VoiceActivityDetector, AudioChunkProcessor}};

pub struct AudioRecorder;

impl AudioRecorder {
    pub fn initialise(mut cx: FunctionContext) -> JsResult<JsUndefined> {
        println!("recording started");

        let mut jack_client = JackClient::new();
        let receiver_arc = jack_client.get_audio_receiver();

        jack_client.start_processing();
        let vad = VoiceActivityDetector::new();
        let sample_rate = 16000;
        let mut processor = AudioChunkProcessor::new(vad, sample_rate);
        
        let start_time = Instant::now();
        let duration = Duration::from_secs(10);
    
        while start_time.elapsed() < duration {
            let receiver_guard = receiver_arc.lock().unwrap(); // Lock the mutex
            match receiver_guard.recv() {
                Ok(audio_data) => {
                    println!("Received audio chunk of length: {}", audio_data.len());
                    let result = processor.process_chunk(audio_data);
                },
                Err(_) => {
                    //Handle disconnection or other errors
                    break;
                }
            }
            
            thread::sleep(Duration::from_millis(10));
        }

        jack_client.stop_processing();
        let mut s= 0;
        for i in processor.finalize().to_vec(){
            write_to_file(i, &format!("file{}.wav", s));
            s+=1;
        }
        
        println!("recording finished");
        //write_to_file(audio);

        Ok(cx.undefined())
    }
}

fn write_to_file<P: AsRef<Path>>(audio: Vec<f32>, file_name: P) {
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: 48000,
        bits_per_sample: 32,
        sample_format: hound::SampleFormat::Float,
    };
    let mut writer = hound::WavWriter::create(file_name, spec).unwrap();
    for t in audio {
        writer.write_sample(t).unwrap();
    }
    writer.finalize().unwrap();
}
