use std::sync::mpsc::Sender;
use std::io;
use jack::PortFlags;

pub fn setup_callback(
    send_audio: Sender<Vec<f32>>
){
       // Create client
       jack::set_logger(jack::LoggerType::None);
       let (client, _status) =
           jack::Client::new("rust_jack_simple", jack::ClientOptions::default()).unwrap();
   
       // Register ports. They will be used in a callback that will be
       // called when new data is available.
       let in_a = client
           .register_port("rust_in_l", jack::AudioIn::default())
           .unwrap();
   
       let system_output_port_name = "system:playback_1"; // Replace with your system's output port name
       let connected_ports = client.ports(None, None , PortFlags::empty());
   
       // Connect all found ports to the input port of our client
       for port in connected_ports {
            if port.contains("input") {continue};
           if let Ok(()) = client.connect_ports_by_name(&port, &in_a.name().unwrap()) {}
       }
   
   
    
        let mut buf = vec![];
   
        let process_callback =  move|c: &jack::Client, ps: &jack::ProcessScope| -> jack::Control {
            let in_a_p = in_a.as_slice(ps);
            buf.extend_from_slice(in_a_p);

            if buf.len() > 48000 * 5{
                send_audio.send(buf.to_vec()).unwrap(); // Send the cloned data
                buf.clear();
            }

            jack::Control::Continue
       };
       let process = jack::contrib::ClosureProcessHandler::new(process_callback);
   
       // Activate the client, which starts the processing.
       let active_client = client.activate_async(Notifications, process).unwrap();
   
       // Wait for user input to quit
       println!("Press enter/return to quit...");
       let mut user_input = String::new();
       io::stdin().read_line(&mut user_input).ok();
   
       //let vec = v.lock().unwrap();
       //read(vec.to_vec()); 
   
       if let Err(err) = active_client.deactivate() {
           eprintln!("JACK exited with error: {err}");
       };

}

use rubato::{Resampler, SincFixedIn, SincInterpolationType, SincInterpolationParameters, WindowFunction};

pub fn resample_to_16000hz(sample: &Vec<f32>, input_sample_rate: f64) -> Vec<f32>{
    let params = SincInterpolationParameters {
        sinc_len: 256,
        f_cutoff: 0.95,
        interpolation: SincInterpolationType::Linear,
        oversampling_factor: 256,
        window: WindowFunction::BlackmanHarris2,
    };

    let mut resampler = SincFixedIn::<f32>::new(
        16000.0 / input_sample_rate,
        2.0,
        params,
        1024,
        1,
    )
    .unwrap();

    //let waves_in = vec![chunk.clone()]; 
    let mut resampled_data: Vec<f32> = Vec::new();

    for chunk in sample.chunks(1024) { // Adjust chunk size as needed
        let waves_in = vec![chunk.to_vec()]; 

        let waves_out = resampler.process(&waves_in, None  ).unwrap();
        resampled_data.extend_from_slice(&waves_out[0]);
    }

    resampled_data

    
}


pub fn convert_audio_chunk(
    send_audio: Sender<Vec<f32>>,
) {

        // Open the audio file.
        let reader = hound::WavReader::open("audio.wav").expect("failed to open file");
        #[allow(unused_variables)]
        let hound::WavSpec {
            channels,
            sample_rate,
            bits_per_sample,
            ..
        } = reader.spec();
        
    
        // Convert the audio to floating point samples.
        let samples: Vec<i16> = reader
            .into_samples::<i16>()
            .map(|x| x.expect("Invalid sample"))
            .collect();
        let mut audio = vec![0.0f32; samples.len().try_into().unwrap()];
        whisper_rs::convert_integer_to_float_audio(&samples, &mut audio).expect("Conversion error");
    
        // Convert audio to 16KHz mono f32 samples, as required by the model.
        // These utilities are provided for convenience, but can be replaced with custom conversion logic.
        // SIMD variants of these functions are also available on nightly Rust (see the docs).
        if channels == 2 {
            audio = whisper_rs::convert_stereo_to_mono_audio(&audio).expect("Conversion error");
        } else if channels != 1 {
            panic!(">2 channels unsupported");
        }
        
        if sample_rate != 16000 {
            panic!("sample rate must be 16KHz");
        }

        
        
}


struct Notifications;

impl jack::NotificationHandler for Notifications {
    // fn thread_init(&self, _: &jack::Client) {
    //     println!("JACK: thread init");
    // }

    /// Not much we can do here, see https://man7.org/linux/man-pages/man7/signal-safety.7.html.
    unsafe fn shutdown(&mut self, _: jack::ClientStatus, _: &str) {}

    fn freewheel(&mut self, _: &jack::Client, is_enabled: bool) {
        // println!(
        //     "JACK: freewheel mode is {}",
        //     if is_enabled { "on" } else { "off" }
        // );
    }

    fn sample_rate(&mut self, _: &jack::Client, srate: jack::Frames) -> jack::Control {
        //println!("JACK: sample rate changed to {srate}");
        jack::Control::Continue
    }

    fn client_registration(&mut self, _: &jack::Client, name: &str, is_reg: bool) {
        // println!(
        //     "JACK: {} client with name \"{}\"",
        //     if is_reg { "registered" } else { "unregistered" },
        //     name
        // );
    }

    fn port_registration(&mut self, _: &jack::Client, port_id: jack::PortId, is_reg: bool) {
        // println!(
        //     "JACK: {} port with id {}",
        //     if is_reg { "registered" } else { "unregistered" },
        //     port_id
        // );
    }

    fn port_rename(
        &mut self,
        _: &jack::Client,
        port_id: jack::PortId,
        old_name: &str,
        new_name: &str,
    ) -> jack::Control {
     //   println!("JACK: port with id {port_id} renamed from {old_name} to {new_name}",);
        jack::Control::Continue
    }

    fn ports_connected(
        &mut self,
        _: &jack::Client,
        port_id_a: jack::PortId,
        port_id_b: jack::PortId,
        are_connected: bool,
    ) {
        // println!(
        //     "JACK: ports with id {} and {} are {}",
        //     port_id_a,
        //     port_id_b,
        //     if are_connected {
        //         "connected"
        //     } else {
        //         "disconnected"
        //     }
        // );
    }

    fn graph_reorder(&mut self, _: &jack::Client) -> jack::Control {
        //println!("JACK: graph reordered");
        jack::Control::Continue
    }

    fn xrun(&mut self, _: &jack::Client) -> jack::Control {
      //  println!("JACK: xrun occurred");
        jack::Control::Continue
    }
}
