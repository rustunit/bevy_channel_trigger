# bevy_channel_trigger

[![Following released Bevy versions](https://img.shields.io/badge/Bevy%20tracking-released%20version-lightblue)](https://bevyengine.org/learn/quick-start/plugin-development/#main-branch-tracking)
[![crates.io](https://img.shields.io/crates/v/bevy_channel_trigger)](https://crates.io/crates/bevy_channel_trigger)
[![docs.rs](https://docs.rs/bevy_channel_trigger/badge.svg)](https://docs.rs/bevy_channel_trigger)

Send events via a channel from anywhere (eg. web-dom, c-ffi) to Bevy Observers. 
Inspired by [bevy_crossbeam_event](https://github.com/johanhelsing/bevy_crossbeam_event) but delivering the events via Bevy Observers instead of `EventReader`.

# example

```rust
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
    app.observe(on_event);

    app.run();
}

// regular bevy observer to handle these events coming into the bevy world
fn on_event(trigger: Trigger<MyEvent>) {
    let event = trigger.event();
    println!("trigger with: {}", event.0);
}
```

# Other examples 

* [bevy_web_drop_image_as_sprite](https://github.com/rustunit/bevy_web_drop_image_as_sprite)

## Our Other Crates

- [bevy_device_lang](https://github.com/rustunit/bevy_device_lang)
- [bevy_ios_review](https://github.com/rustunit/bevy_ios_review)
- [bevy_ios_impact](https://github.com/rustunit/bevy_ios_impact)
- [bevy_ios_alerts](https://github.com/rustunit/bevy_ios_alerts)
- [bevy_ios_iap](https://github.com/rustunit/bevy_ios_iap)
- [bevy_ios_notifications](https://github.com/rustunit/bevy_ios_notifications)
- [bevy_ios_gamecenter](https://github.com/rustunit/bevy_ios_gamecenter)
- [bevy_web_popups](https://github.com/rustunit/bevy_web_popups)
- [bevy_libgdx_asset](https://github.com/rustunit/bevy_libgdx_asset)
- [bevy_debug_log](https://github.com/rustunit/bevy_debug_log)

## Compatible Bevy Versions

|bevy|our version|
|-|-|
|0.14|0.1|

## License

this crate is dual-licensed under either [MIT](https://opensource.org/license/MIT) or [Apache 2.0](https://www.apache.org/licenses/LICENSE-2.0), at your option.
