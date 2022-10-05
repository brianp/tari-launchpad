use anyhow::Error;

use super::{Event, ImageTask, Status};
use crate::{
    config::ManagedProtocol,
    image::checker::{CheckerContext, CheckerEvent},
    task::TaskContext,
    utils::TaskGuard,
};

impl<C: ManagedProtocol> TaskContext<ImageTask<C>> {
    pub fn process_event_impl(&mut self, event: Event) -> Result<(), Error> {
        match event {
            Event::Created => self.on_created(),
            Event::Destroyed => self.on_destroyed(),
            Event::Started => self.on_started(),
            Event::Killed => self.on_killed(),
            Event::Terminated => self.on_terminated(),
            Event::CheckerEvent(event) => self.on_checker_event(event),
        }
    }

    fn on_created(&mut self) -> Result<(), Error> {
        if let Status::WaitContainerCreated = self.status.get() {
            self.status.set(Status::StartContainer);
        }
        Ok(())
    }

    fn on_destroyed(&mut self) -> Result<(), Error> {
        if let Status::WaitContainerRemoved = self.status.get() {
            self.status.set(Status::CleanDangling);
        }
        Ok(())
    }

    fn on_started(&mut self) -> Result<(), Error> {
        if let Status::WaitContainerStarted { .. } = self.status.get() {
            let checker = self.inner.image.checker();
            let logs = self.logs_stream();
            // let stats = self.stats_stream();
            let sender = self.sender().clone();
            let context = CheckerContext::new(logs, sender);
            let fur = checker.entrypoint(context);
            let checker = tokio::spawn(fur).into();
            self.status.set(Status::Started { checker });
            // TODO: Track logs for the ready checker...
        }
        Ok(())
    }

    fn on_killed(&mut self) -> Result<(), Error> {
        Ok(())
    }

    fn on_checker_event(&mut self, event: CheckerEvent) -> Result<(), Error> {
        if let Status::Started { .. } = self.status.get() {
            match event {
                CheckerEvent::Progress(_) => {
                    // TODO: Forward status
                },
                CheckerEvent::Ready => {
                    self.status.set(Status::Ready);
                },
            }
        }
        Ok(())
    }

    fn on_terminated(&mut self) -> Result<(), Error> {
        match self.status.get() {
            Status::WaitContainerKilled => {
                self.status.set(Status::CleanDangling);
            },
            Status::Started { .. } => {
                // TODO: Add waiting interval + fallback
                // self.status.set(Status::CleanDangling);
            },
            _ => {},
        }
        Ok(())
    }
}
