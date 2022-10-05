use anyhow::Error;

use super::{Status, VolumeTask};
use crate::{config::ManagedProtocol, task::TaskContext};

impl<C: ManagedProtocol> TaskContext<VolumeTask<C>> {
    pub async fn process_update_impl(&mut self) -> Result<(), Error> {
        match self.status.get() {
            Status::InitialState => self.do_initial_state().await,
            Status::Checking => self.do_checking().await,
            Status::WaitCreating => self.do_wait_creating().await,
            Status::Active => self.do_active().await,
        }
    }

    async fn do_initial_state(&mut self) -> Result<(), Error> {
        self.status.set(Status::Checking);
        Ok(())
    }

    async fn do_checking(&mut self) -> Result<(), Error> {
        if self.volume_exists().await {
            self.status.set(Status::Active);
        } else {
            self.try_create_volume().await?;
            self.status.set(Status::WaitCreating);
        }
        Ok(())
    }

    async fn do_wait_creating(&mut self) -> Result<(), Error> {
        Ok(())
    }

    async fn do_active(&mut self) -> Result<(), Error> {
        if !self.should_be_active() {
            // self.status.set(Status::Checking);
        }
        Ok(())
    }
}
