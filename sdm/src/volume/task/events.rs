use anyhow::Error;

use super::{Event, Status, VolumeTask};
use crate::{config::ManagedConfig, task::TaskContext};

impl<C: ManagedConfig> TaskContext<VolumeTask<C>> {
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
        Ok(())
    }
}
