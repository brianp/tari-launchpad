use anyhow::Error;

use super::{Event, VolumeTask};
use crate::{config::ManagedConfig, task::TaskContext};

impl<C: ManagedConfig> TaskContext<VolumeTask<C>> {
    pub fn process_event_impl(&mut self, _event: Event) -> Result<(), Error> {
        Ok(())
    }
}
