use std::{marker::PhantomData, pin::Pin};

use anyhow::Error;
use async_trait::async_trait;
use derive_more::{Deref, DerefMut};
use futures::stream::{FusedStream, Stream, StreamExt};
use tokio::{
    select,
    sync::mpsc,
    time::{sleep, Duration},
};

use super::task::Event;
use crate::{image::ManagedProtocol, scope::ControlEvent, task::TaskSender};

#[derive(Debug)]
pub enum CheckerEvent {
    // TODO: Add starging to progress
    // Progress { stages: 10, current_stage: 1, progress: 57 (pct), desc: "Syncing..." },
    // Progress in percents
    Progress(u8),
    Ready,
}

pub struct CheckerContext<P: ManagedProtocol> {
    logs: Logs,
    sender: TaskSender<Event, P>,
}

impl<P: ManagedProtocol> CheckerContext<P> {
    pub(crate) fn new(logs: Logs, sender: TaskSender<Event, P>) -> Self {
        Self { logs, sender }
    }

    /// Reports the task about the progress.
    pub fn report(&self, event: CheckerEvent) -> Result<(), Error> {
        let event = Event::CheckerEvent(event);

        self.sender.send_direct(event)
    }

    /// Notifies all tasks with the inner event.
    pub fn notify(&self, event: P::Inner) -> Result<(), Error> {
        let event = ControlEvent::InnerEvent(event);

        self.sender.send_broadcast(event)
    }
}

#[async_trait]
pub trait ContainerChecker<P: ManagedProtocol>: Send {
    async fn entrypoint(mut self: Box<Self>, mut ctx: CheckerContext<P>) {
        ctx.report(CheckerEvent::Progress(0)).ok();
        loop {
            select! {
                log_event = ctx.logs.next() => {
                    if let Some(Ok(msg)) = log_event {
                        self.on_log_event(msg, &mut ctx).await;
                    }
                }
                _ = sleep(Duration::from_secs(1)) => {
                    if let Err(err) = self.on_interval(&mut ctx).await {
                        log::error!("On interval checker failed: {}", err);
                    }
                }
            }
        }
    }

    async fn on_log_event(&mut self, record: String, ctx: &mut CheckerContext<P>) {}

    async fn on_interval(&mut self, _ctx: &mut CheckerContext<P>) -> Result<(), Error> {
        Ok(())
    }
}

pub struct ReadyIfStarted;

impl<P: ManagedProtocol> ContainerChecker<P> for ReadyIfStarted {}

#[derive(Deref, DerefMut)]
pub struct Logs {
    stream: Pin<Box<dyn FusedStream<Item = Result<String, Error>> + Send>>,
}

impl Logs {
    pub fn new<S>(stream: S) -> Self
    where
        S: Stream<Item = Result<String, Error>>,
        S: Send + 'static,
    {
        Self {
            stream: Box::pin(stream.fuse()),
        }
    }
}

#[derive(Deref, DerefMut)]
pub struct Stats {
    stream: Pin<Box<dyn FusedStream<Item = Result<(), Error>> + Send>>,
}

impl Stats {
    pub fn new<S>(stream: S) -> Self
    where
        S: Stream<Item = Result<(), Error>>,
        S: Send + 'static,
    {
        Self {
            stream: Box::pin(stream.fuse()),
        }
    }
}
