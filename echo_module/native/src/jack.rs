use jack::{AudioIn, Client, ClientOptions, PortFlags};
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use tokio::sync::{mpsc, Mutex};
use tokio::task;

pub struct JackClient {
    running: Arc<AtomicBool>,
    audio_tx: mpsc::Sender<Vec<f32>>,
    audio_rx: Arc<Mutex<mpsc::Receiver<Vec<f32>>>>,
    audio_sources: Vec<String>
}

impl JackClient {
    pub fn new(audio_sources: Vec<String>) -> Self {
        let (audio_tx, audio_rx) = mpsc::channel(32); // Use tokio channel
        JackClient {
            running: Arc::new(AtomicBool::new(false)),
            audio_tx,
            audio_rx: Arc::new(Mutex::new(audio_rx)),
            audio_sources
        }
    }

    pub fn start_processing(&mut self) {
        let running = self.running.clone();
        let audio_tx_clone = self.audio_tx.clone();
        let running_clone = self.running.clone();
        let audio_sources_clone = self.audio_sources.clone();

        task::spawn(async move {
            running.store(true, Ordering::SeqCst);
            jack::set_logger(jack::LoggerType::None);

            let (client, _status) = Client::new("echo", ClientOptions::default()).unwrap();
            let in_a = client.register_port("input", AudioIn::default()).unwrap();
            
            let connected_ports = client.ports(None, None, PortFlags::empty());


            for port in audio_sources_clone {
                if let Err(e) = client.connect_ports_by_name(&port, &in_a.name().unwrap()) {
                    eprintln!("Error connecting ports: {}", e);
                }
            }

            let mut buffer = vec![];
            let process_callback = move |_: &Client, ps: &jack::ProcessScope| -> jack::Control {
                if !running_clone.load(Ordering::SeqCst) {
                    return jack::Control::Quit;
                }

                let in_a_p = in_a.as_slice(ps);

                buffer.extend_from_slice(in_a_p);

                //1s
              //  if buffer.len() > 48000 {
                    if let Err(e) = audio_tx_clone.blocking_send(std::mem::take(&mut buffer)) {
                        eprintln!("Error sending audio data: {}", e);
                    }
               // }

                jack::Control::Continue
            };

            let process = jack::contrib::ClosureProcessHandler::new(process_callback);
            let _active_client = client.activate_async(Notifications, process).unwrap();

            while running.load(Ordering::SeqCst) {
                tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
            }

            println!("Jack Client Thread Finished");
        });
    }

    

    pub fn stop_processing(&mut self) {
        self.running.store(false, Ordering::SeqCst);
    }
    pub fn get_audio_receiver(&self) -> Arc<Mutex<mpsc::Receiver<Vec<f32>>>> {
        self.audio_rx.clone()
    }

}

pub trait AudioDataReceiver {
    fn get_audio_receiver(&self) -> Arc<Mutex<mpsc::Receiver<Vec<f32>>>>;
}

impl AudioDataReceiver for JackClient {
    fn get_audio_receiver(&self) -> Arc<Mutex<mpsc::Receiver<Vec<f32>>>> {
        self.audio_rx.clone()
    }
}

pub fn get_audio_sources() -> Vec<String>{
    let (client, _status) = Client::new("dummy", ClientOptions::default()).unwrap();
    let connected_ports = client.ports(None, None, PortFlags::empty());

    connected_ports
}
struct Notifications;

impl jack::NotificationHandler for Notifications {
    fn thread_init(&self, _: &jack::Client) {
        println!("JACK: thread init");
    }

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
