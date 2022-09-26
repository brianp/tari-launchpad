mod docker;
mod events;
mod update;

use anyhow::Error;
use async_trait::async_trait;

use super::ManagedNetwork;
use crate::{
    config::ManagedConfig,
    error::ParseError,
    task::{RunnableContext, RunnableTask, TaskContext, TaskEvent, TaskStatus},
    utils::TaskGuard,
};

pub struct NetworkTask<C: ManagedConfig> {
    network: Box<dyn ManagedNetwork<Config = C>>,
    events: Option<TaskGuard<()>>,
    network_name: String,
}

impl<C: ManagedConfig> NetworkTask<C> {
    pub fn new(scope: &str, network: Box<dyn ManagedNetwork<Config = C>>) -> Self {
        let network_name = format!("{}_{}", scope, network.network_name());
        Self {
            network,
            events: None,
            network_name,
        }
    }
}

#[async_trait]
impl<C: ManagedConfig> RunnableTask for NetworkTask<C> {
    type Config = C;
    type Event = Event;
    type Status = Status;

    fn name(&self) -> &str {
        self.network_name.as_ref()
    }
}

#[async_trait]
impl<C: ManagedConfig> RunnableContext<NetworkTask<C>> for TaskContext<NetworkTask<C>> {
    async fn initialize(&mut self) {
        self.subscribe_to_events();
    }

    fn reconfigure(&mut self, config: Option<&C>) -> bool {
        self.inner.network.reconfigure(config)
    }

    fn process_event(&mut self, event: Event) -> Result<(), Error> {
        self.process_event_impl(event)
    }

    async fn update(&mut self) -> Result<(), Error> {
        self.process_update_impl().await
    }
}

#[derive(Debug)]
pub enum Status {
    InitialState,
    Cleanup,
    WaitRemoving,
    Inactive,
    WaitCreating,
    Active,
}

impl TaskStatus for Status {
    fn is_ready(&self) -> bool {
        matches!(self, Self::Active)
    }
}

impl Default for Status {
    fn default() -> Self {
        Self::InitialState
    }
}

#[derive(Debug)]
pub enum Event {
    Destroyed,
    Created,
}

impl TaskEvent for Event {}

impl TryFrom<String> for Event {
    type Error = ParseError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        // Docker values!
        match value.as_ref() {
            "destroy" => Ok(Self::Destroyed),
            "create" => Ok(Self::Created),
            _ => Err(ParseError(value)),
        }
    }
}
