use dioxus::prelude::*;
use nultr_client_lib::{
    define_event_channels, ws::controller::dioxus_integration::GetChannelTrait,
};

use crate::error_list::DisplayError;

pub static EVENT_CHANNELS: GlobalSignal<EventChannels> = Global::new(|| EventChannels::default());

define_event_channels! {
    EventChannels (
        DisplayError
    )
}

#[macro_export]
macro_rules! event_listener {
    ($var:ident : $event_type:ty => $body:block) => {
        spawn(async move {
            let mut channels = crate::event::EVENT_CHANNELS.write();
            let rx = <crate::event::EventChannels as nultr_client_lib::ws::controller::dioxus_integration::GetChannelTrait<$event_type>>::get_rx(&mut channels);

            while let Some($var) = rx.recv().await {
                $body
            }
        });
    };
}

pub fn send_event<T>(event: T)
where
    EventChannels: GetChannelTrait<T>,
{
    let mut channels = crate::event::EVENT_CHANNELS.write();
    let tx =
        <EventChannels as nultr_client_lib::ws::controller::dioxus_integration::GetChannelTrait<
            T,
        >>::get_tx(&mut channels);

    tx.send(event).unwrap();
}
