use bevy::{ecs::event::Event, prelude::*};
use flume::{Receiver, Sender, TryRecvError};

#[derive(Resource, Clone, Debug)]
pub struct ChannelSender<T: Event>(Sender<T>);

impl<T: Event> ChannelSender<T> {
    pub fn send(&self, event: impl Into<T>) {
        let event = event.into();
        if let Err(err) = self.0.send(event) {
            error!("sending failed due to receiver being dropped: {err:?}");
        };
    }
}

#[derive(Resource)]
struct EventReceiver<T: Event>(Receiver<T>);

pub trait ChannelTriggerApp {
    fn add_channel_trigger<T: Event>(&mut self) -> ChannelSender<T>;
}

impl ChannelTriggerApp for App {
    fn add_channel_trigger<T: Event>(&mut self) -> ChannelSender<T> {
        let (sender, receiver) = flume::unbounded();
        self.insert_resource(EventReceiver::<T>(receiver));
        self.add_systems(PreUpdate, process_events::<T>);
        ChannelSender::<T>(sender)
    }
}

fn process_events<T: Event>(receiver: Option<Res<EventReceiver<T>>>, mut commands: Commands) {
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
