use std::collections::HashMap;

use anyhow::Error;
use bollard::{
    models::{EventMessage, EventMessageTypeEnum},
    system::EventsOptions,
    volume::{CreateVolumeOptions, RemoveVolumeOptions},
};
use futures::TryStreamExt;

use super::{Event, VolumeTask};
use crate::{
    forwarder::{Converter, Forwarder},
    task::TaskContext,
    volume::ManagedProtocol,
};

impl<C: ManagedProtocol> TaskContext<VolumeTask<C>> {
    pub fn subscribe_to_events(&mut self) {
        let mut type_filter = HashMap::new();
        type_filter.insert("type".to_string(), vec!["volume".to_string()]);
        type_filter.insert("volume".to_string(), vec![self.inner.volume_name.clone()]);
        let opts = EventsOptions {
            since: None,
            until: None,
            filters: type_filter,
        };
        let stream = self.driver.events(Some(opts)).map_err(Error::from);
        let sender = self.sender.clone();
        let conv = EventConv {
            // TODO: Name is not necessary here
            name: self.inner.volume_name.clone(),
        };
        let handle = Forwarder::start(stream, conv, sender);
        self.inner.events = Some(handle);
    }

    pub async fn volume_exists(&mut self) -> bool {
        // TODO: Move to context directly
        self.driver.inspect_volume(&self.inner.volume_name).await.is_ok()
    }

    pub async fn try_create_volume(&mut self) -> Result<(), Error> {
        let options = CreateVolumeOptions {
            name: self.inner.volume_name.as_ref() as &str,
            ..Default::default()
        };
        self.driver.create_volume(options).await?;
        // TODO: Check warnings...
        Ok(())
    }

    pub async fn try_remove_volume(&mut self) -> Result<(), Error> {
        let opts = RemoveVolumeOptions { force: true };
        self.driver.remove_volume(&self.inner.volume_name, Some(opts)).await?;
        Ok(())
    }
}

struct EventConv {
    name: String,
}

impl Converter<EventMessage, Event> for EventConv {
    fn convert(&self, res: Result<EventMessage, Error>) -> Option<Event> {
        if let Ok(EventMessage {
            typ: Some(typ),
            action: Some(action),
            actor: Some(actor),
            ..
        }) = res
        {
            if let Some(attributes) = actor.attributes {
                if let Some(name) = attributes.get("name") {
                    if self.name == *name {
                        // TODO: Check the name
                        match typ {
                            EventMessageTypeEnum::VOLUME => {
                                return action.try_into().ok();
                            },
                            _ => {},
                        }
                    } else {
                        log::error!("Message for other volume {}, but expected {}", name, self.name);
                    }
                }
            }
        }
        None
    }
}
