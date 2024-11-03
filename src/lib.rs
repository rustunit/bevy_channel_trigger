use bevy::{ecs::event::Event, prelude::*};
use flume::{Receiver, Sender, TryRecvError, TrySendError};

#[derive(Resource, Clone, Debug)]
pub struct ChannelSender<T: Event>(Sender<T>);

impl<T: Event> ChannelSender<T> {
    pub fn send(&self, event: impl Into<T>) {
        let event = event.into();
        if let Err(err) = self.0.try_send(event) {
            match err {
                TrySendError::Full(_) => panic!("unable to send event, channel full"),
                TrySendError::Disconnected(_) => {}
            }
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
        self.insert_resource(ChannelSender::<T>(sender.clone()));
        self.insert_resource(EventReceiver::<T>(receiver));
        self.add_systems(PreUpdate, process_events::<T>);
        ChannelSender::<T>(sender)
    }
}

fn process_events<T: Event>(receiver: Res<EventReceiver<T>>, mut commands: Commands) {
    loop {
        match receiver.0.try_recv() {
            Ok(msg) => {
                commands.trigger(msg);
            }
            Err(TryRecvError::Disconnected) => {
                panic!("sender dropped")
            }
            Err(TryRecvError::Empty) => {
                break;
            }
        }
    }
}
