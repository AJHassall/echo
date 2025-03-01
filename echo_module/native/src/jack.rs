use jack::{AudioIn, Client, ClientOptions, PortFlags};
use neon::types::buffer;
use std::thread;
use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        mpsc, Arc, Mutex,
    },
    time::Duration,
};

use crate::audio::AudioDataReceiver;

pub struct JackClient {
    running: Arc<AtomicBool>,
    audio_tx: mpsc::Sender<Vec<f32>>,
    audio_rx: Arc<Mutex<mpsc::Receiver<Vec<f32>>>>, 
    thread_handle: Option<thread::JoinHandle<()>>,
}

impl JackClient {
    pub fn new() -> Self {
        let (audio_tx, audio_rx) = mpsc::channel();
        JackClient {
            running: Arc::new(AtomicBool::new(false)),
            audio_tx,
            audio_rx: Arc::new(Mutex::new(audio_rx)), // Initialize Arc<Mutex<>>,
            thread_handle: None, // Initialize to None
        }
    }
    pub fn start_processing(&mut self) { // Renamed to start_processing
        let running = self.running.clone();
        let audio_tx_clone = self.audio_tx.clone();

        let handle = thread::spawn(move || {
            running.store(true, Ordering::SeqCst);
            jack::set_logger(jack::LoggerType::None);

            let (client, _status) = Client::new("echo", ClientOptions::default()).unwrap();
            let in_a = client.register_port("input", AudioIn::default()).unwrap();

            let connected_ports = client.ports(None, None, PortFlags::empty());

            std::thread::sleep(Duration::from_secs(1));

            for port in connected_ports {
                if let Err(e) = client.connect_ports_by_name(&port, &in_a.name().unwrap()) {
                    eprintln!("Error connecting ports: {}", e);
                }
            }

            let running_clone = running.clone();
            let mut buffer = vec![];
            let process_callback = move |_: &Client, ps: &jack::ProcessScope| -> jack::Control {
                if !running_clone.load(Ordering::SeqCst) {
                    return jack::Control::Quit;
                }

                let in_a_p = in_a.as_slice(ps);
                let audio_data: Vec<f32> = in_a_p.to_vec();

                buffer.extend_from_slice(in_a_p);

                //1ms
                if buffer.len() > 48000 {
                    if let Err(e) = audio_tx_clone.send(buffer.clone()) {
                        eprintln!("Error sending audio data: {}", e);
                    }
                    buffer.clear();
                }

                jack::Control::Continue
            };

            let process = jack::contrib::ClosureProcessHandler::new(process_callback);
            let _active_client = client.activate_async(Notifications, process).unwrap();

            while running.load(Ordering::SeqCst) {
                std::thread::sleep(std::time::Duration::from_millis(10));
            }
        });

        self.thread_handle = Some(handle); // Store the thread handle
    }

    pub fn stop_processing(&mut self) { 
        self.running.store(false, Ordering::SeqCst);
        if let Some(handle) = self.thread_handle.take() { 
            handle.join().unwrap();
        }
    }
    

}

impl AudioDataReceiver for JackClient {
    fn get_audio_receiver(&self) -> Arc<Mutex<mpsc::Receiver<Vec<f32>>>> {
        self.audio_rx.clone()
    }
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
