use anyhow::Error;

use super::{NetworkTask, Status};
use crate::{config::ManagedConfig, task::TaskContext};

impl<C: ManagedConfig> TaskContext<NetworkTask<C>> {
    pub async fn process_update_impl(&mut self) -> Result<(), Error> {
        match self.status.get() {
            Status::InitialState => self.do_initial_state().await,
            Status::Cleanup => self.do_cleanup().await,
            Status::WaitRemoving => self.do_wait_removing().await,
            Status::Inactive => self.do_inactive().await,
            Status::WaitCreating => self.do_wait_creating().await,
            Status::Active => self.do_active().await,
        }
    }

    async fn do_initial_state(&mut self) -> Result<(), Error> {
        self.status.set(Status::Cleanup);
        Ok(())
    }

    async fn do_cleanup(&mut self) -> Result<(), Error> {
        if self.network_exists().await {
            self.try_remove_network().await?;
            self.status.set(Status::WaitRemoving);
        } else {
            self.status.set(Status::Inactive);
        }
        Ok(())
    }

    async fn do_wait_removing(&mut self) -> Result<(), Error> {
        Ok(())
    }

    async fn do_inactive(&mut self) -> Result<(), Error> {
        if self.should_be_active() {
            self.try_create_network().await?;
            self.status.set(Status::WaitCreating);
        }
        Ok(())
    }

    async fn do_wait_creating(&mut self) -> Result<(), Error> {
        Ok(())
    }

    async fn do_active(&mut self) -> Result<(), Error> {
        if !self.should_be_active() {
            self.status.set(Status::Cleanup);
        }
        Ok(())
    }
}
