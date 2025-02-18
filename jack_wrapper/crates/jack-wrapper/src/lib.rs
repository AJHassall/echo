use neon::prelude::*;

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("Start", start_recording)?;
    cx.export_function("Stop", stop_recording)?;
    Ok(())
}


use std::{future, io, sync::{atomic::{AtomicBool, Ordering}, mpsc::channel, Arc, Mutex}, thread};
use jack::{AudioIn, Client, ClientOptions, PortFlags};
use neon::{handle, prelude::*};
use hound::{WavWriter, WavSpec, SampleFormat};

//use once_cell::sync::OnceCell;

static RUNNING: AtomicBool = AtomicBool::new(false);
static AUDIO_DATA: Mutex<Vec<f32>> = Mutex::new(Vec::new());

 fn start_recording(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    if RUNNING.load(Ordering::SeqCst) {
        return Ok(cx.undefined()); // Already running
    }

    RUNNING.store(true, Ordering::SeqCst);

    thread::spawn(move || {
        jack::set_logger(jack::LoggerType::None);

        let (client, _status) = Client::new("echo", ClientOptions::default()).unwrap();
        let in_a = client.register_port("input", AudioIn::default()).unwrap();

        let connected_ports = client.ports(None, None, PortFlags::empty());
        for port in connected_ports {
            if let Err(e) = client.connect_ports_by_name(&port, &in_a.name().unwrap()) {
                eprintln!("Error connecting ports: {}", e);
            }
        }

        let process_callback = move |_: &Client, ps: &jack::ProcessScope| -> jack::Control {
            if !RUNNING.load(Ordering::SeqCst) {
                return jack::Control::Quit; // Stop processing when flag is set
            }

            let in_a_p = in_a.as_slice(ps);
            let mut audio_data = AUDIO_DATA.lock().unwrap();
            audio_data.extend_from_slice(in_a_p);

            jack::Control::Continue
        };

        let process = jack::contrib::ClosureProcessHandler::new(process_callback);
        let _active_client = client.activate_async(Notifications, process).unwrap();

        while RUNNING.load(Ordering::SeqCst) {
            std::thread::sleep(std::time::Duration::from_millis(10)); // Prevent CPU hogging
        }

        // Save audio data to WAV file
        let audio_data = AUDIO_DATA.lock().unwrap();
        let spec = WavSpec {
            channels: 1, // Mono audio
            sample_rate: 48000, // Or whatever your sample rate is
            bits_per_sample: 32, // 32-bit float
            sample_format: SampleFormat::Float,
        };

        let mut writer = WavWriter::create("audio.wav", spec).unwrap();
        for &sample in audio_data.iter() {
            writer.write_sample(sample).unwrap();
        }
        writer.finalize().unwrap();


        _active_client.deactivate().unwrap();
        //client.().unwrap();
        AUDIO_DATA.lock().unwrap().clear(); // Clear data after saving
    });

    Ok(cx.undefined())
}
 fn stop_recording(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    RUNNING.store(false, Ordering::SeqCst);
    Ok(cx.undefined())
}
struct Notifications;

impl jack::NotificationHandler for Notifications {
    fn thread_init(&self, _: &jack::Client) {
        println!("JACK: thread init");
    }

    /// Not much we can do here, see https://man7.org/linux/man-pages/man7/signal-safety.7.html.
    unsafe fn shutdown(&mut self, _: jack::ClientStatus, _: &str) {}

    fn freewheel(&mut self, _: &jack::Client, is_enabled: bool) {
        println!(
            "JACK: freewheel mode is {}",
            if is_enabled { "on" } else { "off" }
        );
    }

    fn sample_rate(&mut self, _: &jack::Client, srate: jack::Frames) -> jack::Control {
        println!("JACK: sample rate changed to {srate}");
        jack::Control::Continue
    }

    fn client_registration(&mut self, _: &jack::Client, name: &str, is_reg: bool) {
        println!(
            "JACK: {} client with name \"{}\"",
            if is_reg { "registered" } else { "unregistered" },
            name
        );
    }

    fn port_registration(&mut self, _: &jack::Client, port_id: jack::PortId, is_reg: bool) {
        println!(
            "JACK: {} port with id {}",
            if is_reg { "registered" } else { "unregistered" },
            port_id
        );
    }

    fn port_rename(
        &mut self,
        _: &jack::Client,
        port_id: jack::PortId,
        old_name: &str,
        new_name: &str,
    ) -> jack::Control {
        println!("JACK: port with id {port_id} renamed from {old_name} to {new_name}",);
        jack::Control::Continue
    }

    fn ports_connected(
        &mut self,
        _: &jack::Client,
        port_id_a: jack::PortId,
        port_id_b: jack::PortId,
        are_connected: bool,
    ) {
        println!(
            "JACK: ports with id {} and {} are {}",
            port_id_a,
            port_id_b,
            if are_connected {
                "connected"
            } else {
                "disconnected"
            }
        );
    }

    fn graph_reorder(&mut self, _: &jack::Client) -> jack::Control {
        println!("JACK: graph reordered");
        jack::Control::Continue
    }

    fn xrun(&mut self, _: &jack::Client) -> jack::Control {
        println!("JACK: xrun occurred");
        jack::Control::Continue
    }
}