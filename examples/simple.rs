use bevy::prelude::*;
use std::{thread, time::Duration};

#[derive(Event)]
struct MyEvent(i32);

fn main() {
    use bevy_channel_trigger::ChannelTriggerApp;

    let mut app = App::new();
    app.add_plugins(MinimalPlugins);

    // create channel
    let sender = app.add_channel_trigger::<MyEvent>();

    // use sender from anywhere:
    thread::spawn(move || {
        let mut counter = 1;
        loop {
            // send events back to bevy
            sender.send(MyEvent(counter));
            thread::sleep(Duration::from_secs(1));
            counter += 1;
        }
    });

    // register an observer to receive the events sent via `sender`
    app.add_observer(on_event);

    app.run();
}

// regular bevy observer to handle these events coming into the bevy world
fn on_event(trigger: On<MyEvent>) {
    let event = trigger.event();
    println!("trigger with: {}", event.0);
}
