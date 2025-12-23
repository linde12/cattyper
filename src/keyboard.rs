use evdev::Device;
use evdev::EventType;
use futures::SinkExt;
use futures::Stream;

use crate::{KeyboardEventType, Message};

pub fn keyboard_event_stream() -> impl Stream<Item = Message> {
    iced::stream::channel(100, |output| async move {
        let devices = get_keyboard_devices();
        let event_streams = devices
            .into_iter()
            .map(|device| device.into_event_stream().unwrap());

        // listen to all keyboard devices concurrently
        for mut stream in event_streams {
            let mut output = output.clone();
            tokio::spawn(async move {
                loop {
                    let ev = stream.next_event().await.unwrap();
                    match &ev.event_type() {
                        &EventType::KEY => {
                            let key_state = match ev.value() {
                                0 => KeyboardEventType::Released,
                                _ => KeyboardEventType::Pressed,
                            };
                            output
                                .send(Message::KeyPress(key_state.clone()))
                                .await
                                .unwrap();
                        }
                        _ => continue,
                    }
                }
            });
        }
    })
}

fn get_keyboard_devices() -> Vec<Device> {
    std::fs::read_dir("/dev/input")
        .ok()
        .unwrap()
        .filter(|entry| {
            let entry = entry.as_ref().ok().unwrap();
            let path = entry.path();
            path.to_str().unwrap().starts_with("/dev/input/event")
        })
        .filter_map(|entry| {
            let entry = entry.ok().unwrap();
            let path = entry.path();
            Device::open(&path).ok()
        })
        .filter(|device| device.supported_events().contains(EventType::KEY))
        .collect()
}
