use std::pin::Pin;

use anyhow::Error;
use async_trait::async_trait;
use derive_more::{Deref, DerefMut};
use futures::stream::{FusedStream, Stream, StreamExt};
use tokio::{select, sync::mpsc};

use super::task::Event;

#[derive(Debug)]
pub enum CheckerEvent {
    // Progress in percents
    Progress(u8),
    Ready,
}

pub struct CheckerContext {
    logs: Logs,
    sender: mpsc::UnboundedSender<Event>,
}

impl CheckerContext {
    pub(crate) fn new(logs: Logs, sender: mpsc::UnboundedSender<Event>) -> Self {
        Self { logs, sender }
    }

    pub fn send(&self, event: CheckerEvent) -> Result<(), Error> {
        let event = Event::CheckerEvent(event);
        self.sender
            .send(event)
            .map_err(|_| Error::msg("Can't send message from the checker's context"))?;
        Ok(())
    }
}

#[async_trait]
pub trait ContainerChecker: Send {
    async fn entrypoint(mut self: Box<Self>, mut ctx: CheckerContext) {
        loop {
            select! {
                log_event = ctx.logs.next() => {
                    if let Some(Ok(msg)) = log_event {
                        self.on_log_event(msg, &mut ctx).await;
                    } else {
                        break;
                    }
                }
            }
        }
    }

    async fn on_log_event(&mut self, record: String, ctx: &mut CheckerContext) {}
}

pub struct ReadyIfStarted;

impl ContainerChecker for ReadyIfStarted {}

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