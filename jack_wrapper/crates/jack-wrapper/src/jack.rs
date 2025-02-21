use jack::{AudioIn, Client, ClientOptions, PortFlags};
use std::sync::{
    atomic::{AtomicBool, Ordering},
    mpsc, Arc, Mutex,
};
use std::thread;

pub struct JackClient {
    running: Arc<AtomicBool>,
    audio_data: Arc<Mutex<Vec<f32>>>,
    sender: mpsc::Sender<Vec<f32>>, // For sending audio data to the stream,
    receiver: Arc<Mutex<mpsc::Receiver<Vec<f32>>>>, // Add the receiver field
}

impl JackClient {
    pub fn new() -> Self {
        let (sender, receiver) = mpsc::channel(); // Choose a suitable buffer size
        JackClient {
            running: Arc::new(AtomicBool::new(false)),
            audio_data: Arc::new(Mutex::new(Vec::new())),
            sender,
            receiver: Arc::new(Mutex::new(receiver))
        }
    }
    pub fn start_recording(&self) {
        if self.running.load(Ordering::SeqCst) {
            //return Ok(cx.undefined()); // Already running
        }

        self.running.store(true, Ordering::SeqCst);
        
        let audio_data = self.audio_data.clone(); 
        let sender = self.sender.clone(); // Clone the sender for the callback
        let running = self.running.clone();
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
                // if !running.load(Ordering::SeqCst) {
                //     return jack::Control::Quit; // Stop processing when flag is set
                // }

                let in_a_p = in_a.as_slice(ps);
                let mut audio_data_lock = audio_data.lock().unwrap();
                audio_data_lock.extend_from_slice(in_a_p);

                // Send the audio data through the channel
                if let Err(e) = sender.send(audio_data_lock.clone()) { // Clone data to send
                    eprintln!("Error sending audio data: {}", e);
                    return jack::Control::Continue; // Or handle the error appropriately
                }
                audio_data_lock.clear(); // Clear the buffer after sending

                jack::Control::Continue
            };

            let process = jack::contrib::ClosureProcessHandler::new(process_callback);
            let _active_client = client.activate_async(Notifications, process).unwrap();

            while running.load(Ordering::SeqCst) {
                std::thread::sleep(std::time::Duration::from_millis(10)); // Prevent CPU hogging
            }

   
           // self.audio_data.lock().unwrap().clear(); // Clear data after saving
        });

        
    }
   pub fn stop_recording(self) {
        self.running.store(false, Ordering::SeqCst);
    }
    pub fn get_receiver(&self) -> Arc<Mutex<mpsc::Receiver<Vec<f32>>>> {
        self.receiver.clone() // Increment the Arc's strong count
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
