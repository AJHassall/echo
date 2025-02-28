// event_publisher.rs
use lazy_static::lazy_static;
use neon::prelude::*;
use std::sync::{Arc, Mutex};

lazy_static! {
    static ref EVENT_PUBLISHER: Mutex<Option<EventPublisher>> = Mutex::new(None);
}
pub struct EventPublisher {
    callback: Arc<Mutex<Root<JsFunction>>>,
    channel: neon::event::Channel,
}

impl EventPublisher {
    fn new(callback: Arc<Mutex<Root<JsFunction>>>, channel: neon::event::Channel) -> Self {
        EventPublisher { callback, channel }
    }

    fn publish(&self, event_type: String, message: String, event_id: String) {
        let callback_arc_clone = self.callback.clone();
        let channel = self.channel.clone();

        channel.send(move |mut cx| {
            let callback_guard = callback_arc_clone.lock().unwrap();
            let callback_clone = callback_guard.clone(&mut cx);
            let callback = callback_clone.into_inner(&mut cx);
            let this = cx.undefined();

            let event = cx.empty_object();
            let event_type = cx.string(event_type);
            let message = cx.string(message);
            let event_id = cx.string(event_id);

            event.set(&mut cx, "event_type", event_type)?;
            event.set(&mut cx, "message", message)?;
            event.set(&mut cx, "event_id", event_id)?;

            //let message = cx.string(&data.clone()).upcast();
            let args = vec![event.upcast()];

            callback.call(&mut cx, this, args)?;

            Ok(())
        });
    }

    pub fn publish_if_available(event_type: String, message: String, event_id: String) {
        if let Some(guard) = Self::get_event_publisher() {
            if let Some(publisher) = guard.as_ref() {
                publisher.publish(event_type, message, event_id);
            }
        }
    }

    pub fn get_event_publisher() -> Option<std::sync::MutexGuard<'static, Option<EventPublisher>>> {
        EVENT_PUBLISHER.lock().ok()
    }
}

pub fn initialise_event_publisher(
    callback: Arc<Mutex<Root<JsFunction>>>,
    channel: neon::event::Channel,
) {
    let publisher = EventPublisher::new(callback, channel);

    let mut guard = EVENT_PUBLISHER.lock().unwrap();
    *guard = Some(publisher);
}
