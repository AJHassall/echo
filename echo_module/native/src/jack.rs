use jack::{
    contrib::ClosureProcessHandler, AsyncClient, AudioIn, Client, ClientOptions, Control, Error,
    Port, PortFlags, ProcessScope,
};
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use tokio::sync::{mpsc, Mutex};
use tokio::task;

type MyProcessCallback = Box<dyn FnMut(&Client, &ProcessScope) -> Control + Send + Sync + 'static>; // Box the trait object

// Define a type alias for your specific AsyncClient type, using the boxed callback type
type MyAsyncClient = AsyncClient<Notifications, ClosureProcessHandler<(), MyProcessCallback>>;

pub struct JackClient {
    running: Arc<AtomicBool>,
    audio_tx: mpsc::Sender<Vec<f32>>,
    audio_rx: Arc<Mutex<mpsc::Receiver<Vec<f32>>>>,
    active_client: Arc<Mutex<Option<MyAsyncClient>>>, // Store AsyncClient Option
    input_port: Arc<Mutex<Option<Port<AudioIn>>>>,    // Store input port
}

impl JackClient {
    pub fn new() -> Self {
        let (audio_tx, audio_rx) = mpsc::channel(32); // Use tokio channel
        JackClient {
            running: Arc::new(AtomicBool::new(false)),
            audio_tx,
            audio_rx: Arc::new(Mutex::new(audio_rx)),
            active_client: Arc::new(Mutex::new(None)), // Initialize active_client as None
            input_port: Arc::new(Mutex::new(None)),    //
        }
    }

    async fn setup_ports(&self, client: &Client) -> Result<(), Error> {
        let in_a = client.register_port("input", AudioIn::default())?;
        let connected_ports = client.ports(None, None, PortFlags::empty());

        for port in connected_ports {
            if let Err(e) = client.connect_ports_by_name(&port, &in_a.name().unwrap()) {
                eprintln!("Error connecting ports: {}", e);
            }
        }

        let mut input_port_guard = self.input_port.lock().await;
        *input_port_guard = Some(in_a); // Store the registered input port
        Ok(())
    }

    pub fn start_processing(&mut self) {
        let running = self.running.clone();
        let audio_tx_clone = self.audio_tx.clone();
        let running_clone = self.running.clone();
        let active_client_clone = self.active_client.clone();

        task::spawn(async move {
            running.store(true, Ordering::SeqCst);
            jack::set_logger(jack::LoggerType::None);

            let (client, _status) = Client::new("echo", ClientOptions::default()).unwrap();
            //let in_a = client.register_port("input", AudioIn::default()).unwrap();

            let connected_ports = client.ports(None, None, PortFlags::empty());

            for port in connected_ports {
                if let Err(e) = client.connect_ports_by_name(&port, &in_a.name().unwrap()) {
                    eprintln!("Error connecting ports: {}", e);
                }
            }

            let mut buffer = vec![];
            let process_callback = move |_: &Client, ps: &ProcessScope| -> jack::Control {
                if !running_clone.load(Ordering::SeqCst) {
                    return jack::Control::Quit;
                }

                // Access the input port through the Arc<Mutex<Option<Port<AudioIn>>>>
                let input_port_guard = input_port_clone.lock().await;
                if let Some(input_port) = &*input_port_guard {
                    let in_a_p = input_port.as_slice(ps);
                    buffer.extend_from_slice(in_a_p);

                    if let Err(e) = audio_tx_clone.blocking_send(std::mem::take(&mut buffer)) {
                        eprintln!("Error sending audio data: {}", e);
                    }
                } else {
                    eprintln!("Input port is not registered in process callback!");
                    return jack::Control::Quit; // Stop processing if input port is unexpectedly missing
                }


                jack::Control::Continue
            };

            let process: ClosureProcessHandler<(), MyProcessCallback> =
                ClosureProcessHandler::new(Box::new(process_callback));
            match client.activate_async(Notifications, process) {
                Ok(active_client) => {
                    println!("Jack Client Activated Async");
                    let mut active_client_guard = active_client_clone.lock().await;
                    let option_ref = &mut *active_client_guard;
                    *option_ref = Some(active_client);
                }
                Err(e) => {
                    eprintln!("Error activating client async: {}", e);
                    running.store(false, Ordering::SeqCst);
                }
            }

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
