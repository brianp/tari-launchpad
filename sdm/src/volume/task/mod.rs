mod docker;
mod events;
mod update;

use anyhow::Error;
use async_trait::async_trait;

use super::ManagedVolume;
use crate::{
    config::ManagedProtocol,
    error::ParseError,
    task::{RunnableContext, RunnableTask, TaskContext, TaskEvent, TaskStatus},
    utils::TaskGuard,
};

pub struct VolumeTask<C: ManagedProtocol> {
    events: Option<TaskGuard<()>>,
    volume: Box<dyn ManagedVolume<Protocol = C>>,

    volume_name: String,
}

impl<C: ManagedProtocol> VolumeTask<C> {
    pub fn new(scope: &str, volume: Box<dyn ManagedVolume<Protocol = C>>) -> Self {
        let volume_name = format!("{}_{}", scope, volume.volume_name());
        Self {
            events: None,
            volume,
            volume_name,
        }
    }
}

#[async_trait]
impl<C: ManagedProtocol> RunnableTask for VolumeTask<C> {
    type Event = Event;
    type Protocol = C;
    type Status = Status;

    fn name(&self) -> &str {
        self.volume_name.as_ref()
    }
}

#[async_trait]
impl<C: ManagedProtocol> RunnableContext<VolumeTask<C>> for TaskContext<VolumeTask<C>> {
    async fn initialize(&mut self) {
        self.subscribe_to_events();
    }

    fn reconfigure(&mut self, config: Option<&C::Config>) -> bool {
        self.inner.volume.reconfigure(config)
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
    Checking,
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
