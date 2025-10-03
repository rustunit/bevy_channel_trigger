use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use bevy_log::prelude::*;
use crossbeam_channel::{Receiver, Sender, TryRecvError};

/// channel sender to share with multiple producers and offering a simple `send` function
#[derive(Resource, Clone, Debug)]
pub struct ChannelSender<T: Event>(Sender<T>);

impl<T: Event> ChannelSender<T> {
    /// send `event` to our central receiver that forwards them as triggers that can be observed
    pub fn send(&self, event: impl Into<T>) {
        let event = event.into();
        if let Err(err) = self.0.send(event) {
            error!("sending failed due to receiver being dropped: {err:?}");
        };
    }
}

#[derive(Resource)]
struct EventReceiver<T: Event>(Receiver<T>);

/// Extension to Bevy `App` that allows ergonomic creation of the channel
pub trait ChannelTriggerApp {
    /// Spawns a channel registers the receiver as a resource and returns the `ChannelSender<T>`
    /// This sender can be used from anywhere to send events into the Bevy world.
    /// These triggers can be subscribed to via `app.observe`.
    fn add_channel_trigger<'a, T: Event>(&mut self) -> ChannelSender<T>
    where
        <T as bevy_ecs::event::Event>::Trigger<'a>: std::default::Default;
}

impl ChannelTriggerApp for App {
    fn add_channel_trigger<'a, T: Event>(&mut self) -> ChannelSender<T>
    where
        <T as bevy_ecs::event::Event>::Trigger<'a>: std::default::Default,
    {
        let (sender, receiver) = crossbeam_channel::unbounded();
        self.insert_resource(EventReceiver::<T>(receiver));
        self.add_systems(PreUpdate, process_events::<T>);
        ChannelSender::<T>(sender)
    }
}

fn process_events<'a, T: Event>(receiver: Option<Res<EventReceiver<T>>>, mut commands: Commands)
where
    <T as bevy_ecs::event::Event>::Trigger<'a>: std::default::Default,
{
    if let Some(receiver) = receiver.as_ref() {
        loop {
            match receiver.0.try_recv() {
                Ok(msg) => {
                    commands.trigger(msg);
                }
                Err(TryRecvError::Disconnected) => {
                    error!("sender dropped, removing receiver");
                    commands.remove_resource::<EventReceiver<T>>();
                }
                Err(TryRecvError::Empty) => {
                    break;
                }
            }
        }
    }
}
