use std::{fmt, pin::Pin};

use anyhow::Error;
use futures::{Stream, StreamExt};
use tokio::sync::mpsc;

use crate::utils::TaskGuard;

pub trait Converter<I, O>: Sync + Send + 'static {
    fn convert(&self, res: Result<I, Error>) -> Option<O>;
}

pub struct Forwarder<I, O> {
    stream: Pin<Box<dyn Stream<Item = Result<I, Error>> + Send>>,
    converter: Box<dyn Converter<I, O>>,
    sender: mpsc::UnboundedSender<O>,
}

impl<I, O> Forwarder<I, O>
where
    I: Send + 'static,
    I: fmt::Debug,
    O: Send + 'static,
    O: fmt::Debug,
{
    pub fn start<S, C>(stream: S, converter: C, sender: mpsc::UnboundedSender<O>) -> TaskGuard<()>
    where
        S: Stream<Item = Result<I, Error>>,
        S: Send + 'static,
        C: Converter<I, O>,
    {
        let this = Self {
            sender,
            converter: Box::new(converter),
            stream: stream.boxed(),
        };
        tokio::spawn(this.entrypoint()).into()
    }

    async fn entrypoint(mut self) {
        while let Some(event) = self.stream.next().await {
            log::trace!("Event in forwarder: {:?}", event);
            if let Some(sdm_event) = self.converter.convert(event) {
                log::debug!("Sending event: {:?}", sdm_event);
                if self.sender.send(sdm_event).is_err() {
                    break;
                }
            }
        }
    }
}
