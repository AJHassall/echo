use std::time::{Duration, Instant};

use neon::{handle, prelude::*};
use tokio::{runtime::Runtime, sync::mpsc, time::error::Error};

use crate::jack::JackClient;



pub struct AudioRecorder;

impl AudioRecorder {

    pub fn initialise(mut cx: FunctionContext) -> JsResult<JsUndefined> {

        println!("recording started");
        let rt  = match Runtime::new()  {
                Ok(rt) => rt,
                Err(_) => {return Ok(cx.undefined());
            }
        };

        let _guard = rt.enter();


        rt.block_on(async {
            tokio::spawn(async move{

                let client = &JackClient::new();
                client.start_recording();

                let receiver = client.get_receiver(); // Get the receiver

                // Process audio data in a separate loop (can be in a different thread)
                let recording_duration = Duration::from_secs(10); // Set your desired duration
                let start_time = Instant::now();

                let mut audio = vec![];
                while start_time.elapsed() < recording_duration {
                    let lock = receiver.lock();
                   // let audio_data = lock.expect("").recv().expect("");
                    match lock.expect("msg").recv() {
                        Ok(audio_data) => {
                            audio.extend(&audio_data);

                            let sum: f32 = audio_data.iter().sum();
                            println!("count {}", audio_data.iter().count());
                            println!("sum: {}", sum);
                            // Process the audio data here
                        }
                        Err(_) => {
                            println!("Channel disconnected. Exiting.");
                            break;
                        }
                    }
                }

                println!("recording finished");
                write_to_file(audio);
            });

        });


        Ok(cx.undefined())
    }
}


fn write_to_file(audio: Vec<f32>){
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: 48000,
        bits_per_sample: 32,
        sample_format: hound::SampleFormat::Float,
    };
    let mut writer = hound::WavWriter::create("sine.wav", spec).unwrap();
    for t in audio{
      
        writer.write_sample(t).unwrap();
    }
    writer.finalize().unwrap();
}