use std::collections::HashMap;

use anyhow::Error;
use bollard::{
    models::{EventMessage, EventMessageTypeEnum},
    network::{CreateNetworkOptions, InspectNetworkOptions},
    system::EventsOptions,
};
use futures::TryStreamExt;

use super::{Event, NetworkTask};
use crate::{
    config::ManagedConfig,
    forwarder::{Converter, Forwarder},
    task::TaskContext,
};

impl<C: ManagedConfig> TaskContext<NetworkTask<C>> {
    pub fn subscribe_to_events(&mut self) {
        let mut type_filter = HashMap::new();
        type_filter.insert("type".to_string(), vec!["network".to_string()]);
        type_filter.insert("network".to_string(), vec![self.inner.network_name.clone()]);
        let opts = EventsOptions {
            since: None,
            until: None,
            filters: type_filter,
        };
        let stream = self.driver.events(Some(opts)).map_err(Error::from);
        let sender = self.sender.clone();
        let conv = EventConv {
            // TODO: Name is not necessary here
            name: self.inner.network_name.clone(),
        };
        let handle = Forwarder::start(stream, conv, sender);
        self.inner.events = Some(handle);
    }

    pub async fn network_exists(&mut self) -> bool {
        let opts = InspectNetworkOptions {
            verbose: false,
            scope: "local",
        };
        self.driver
            .inspect_network(&self.inner.network_name, Some(opts))
            .await
            .is_ok()
    }

    pub async fn try_create_network(&mut self) -> Result<(), Error> {
        let options = CreateNetworkOptions {
            name: self.inner.network_name.as_ref(),
            check_duplicate: true,
            driver: "bridge",
            internal: false,
            attachable: false,
            ingress: false,
            ipam: Default::default(),
            enable_ipv6: false,
            options: Default::default(),
            labels: Default::default(),
        };
        self.driver.create_network(options).await?;
        // TODO: Check warnings...
        Ok(())
    }

    pub async fn try_remove_network(&mut self) -> Result<(), Error> {
        self.driver.remove_network(&self.inner.network_name).await?;
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
                            EventMessageTypeEnum::NETWORK => {
                                return action.try_into().ok();
                            },
                            _ => {},
                        }
                    } else {
                        log::error!("Message for other network {}, but expected {}", name, self.name);
                    }
                }
            }
        }
        None
    }
}
