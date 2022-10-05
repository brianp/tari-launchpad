use anyhow::Error;

use super::{Event, NetworkTask, Status};
use crate::{config::ManagedProtocol, task::TaskContext};

impl<C: ManagedProtocol> TaskContext<NetworkTask<C>> {
    pub fn process_event_impl(&mut self, event: Event) -> Result<(), Error> {
        match event {
            Event::Created => self.on_created(),
            Event::Destroyed => self.on_destroyed(),
        }
    }

    fn on_created(&mut self) -> Result<(), Error> {
        if let Status::WaitCreating = self.status.get() {
            self.status.set(Status::Active);
        }
        Ok(())
    }

    fn on_destroyed(&mut self) -> Result<(), Error> {
        if let Status::WaitRemoving = self.status.get() {
            self.status.set(Status::Inactive);
        }
        Ok(())
    }
}
