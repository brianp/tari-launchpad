mod docker;
mod events;
mod update;

use anyhow::Error;
use async_trait::async_trait;

use super::{checker::CheckerEvent, ManagedContainer};
use crate::{
    config::ManagedConfig,
    error::ParseError,
    task::{RunnableContext, RunnableTask, TaskContext, TaskEvent, TaskStatus},
    utils::TaskGuard,
};

pub struct ImageTask<C: ManagedConfig> {
    events: Option<TaskGuard<()>>,
    container_name: String,
    // TODO: Rename to `fqdn`
    image_name: String,
    image: Box<dyn ManagedContainer<Config = C>>,
}

impl<C: ManagedConfig> ImageTask<C> {
    pub fn new(scope: &str, image: Box<dyn ManagedContainer<Config = C>>) -> Self {
        // let required = image.deps().into_iter().collect();
        let image_name = format!("{}/{}:{}", image.registry(), image.image_name(), image.tag());
        let container_name = format!("{}_{}", scope, image.image_name());
        Self {
            events: None,
            container_name,
            image_name,
            image,
        }
    }
}

#[async_trait]
impl<C: ManagedConfig> RunnableTask for ImageTask<C> {
    type Config = C;
    type Event = Event;
    type Status = Status;

    fn name(&self) -> &str {
        self.container_name.as_ref()
    }
}

#[async_trait]
impl<C: ManagedConfig> RunnableContext<ImageTask<C>> for TaskContext<ImageTask<C>> {
    async fn initialize(&mut self) {
        self.subscribe_to_events();
    }

    fn reconfigure(&mut self, config: Option<&C>) -> bool {
        self.inner.image.reconfigure(config)
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

    PullingImage {
        progress: TaskGuard<()>,
    },

    CleanDangling,
    WaitContainerKilled,
    WaitContainerRemoved,

    CreateContainer,
    WaitContainerCreated,

    StartContainer,
    WaitContainerStarted,

    /// Check the `active` flag
    Idle,

    Started {
        checker: TaskGuard<()>,
    },

    Ready,
}

impl TaskStatus for Status {
    fn is_ready(&self) -> bool {
        matches!(self, Self::Ready)
    }
}

impl Default for Status {
    fn default() -> Self {
        Self::InitialState
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum ContainerState {
    Running,
    NotRunning,
    NotFound,
}

#[derive(Debug)]
pub enum Event {
    Destroyed,
    Created,
    Started,
    Killed,
    Terminated,
    CheckerEvent(CheckerEvent),
}

impl TaskEvent for Event {}

impl TryFrom<String> for Event {
    type Error = ParseError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        // Docker values!
        match value.as_ref() {
            "destroy" => Ok(Self::Destroyed),
            "create" => Ok(Self::Created),
            "start" => Ok(Self::Started),
            "kill" => Ok(Self::Killed),
            "die" => Ok(Self::Terminated),
            _ => Err(ParseError(value)),
        }
    }
}
