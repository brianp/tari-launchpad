use anyhow::Error;

use super::{ContainerState, ImageTask, Status};
use crate::{config::ManagedProtocol, task::TaskContext};

impl<C: ManagedProtocol> TaskContext<ImageTask<C>> {
    pub async fn process_update_impl(&mut self) -> Result<(), Error> {
        match self.status.get() {
            Status::InitialState => self.do_initial_state().await,
            Status::CleanDangling => self.do_clean_dangling().await,
            Status::WaitContainerKilled => self.do_wait_container_killed().await,
            Status::WaitContainerRemoved => self.do_wait_container_removed().await,
            Status::Idle => self.do_idle().await,
            Status::CreateContainer => self.do_create_container().await,
            Status::WaitContainerCreated => self.do_wait_container_created().await,
            Status::StartContainer => self.do_start_container().await,
            Status::WaitContainerStarted => self.do_wait_container_started().await,
            Status::Started { .. } => self.do_started().await,
            Status::Ready => self.do_started().await,
            other => {
                log::warn!("Not implemented state: {:?}", other);
                Ok(())
            },
        }
    }

    async fn do_initial_state(&mut self) -> Result<(), Error> {
        log::debug!("Cheking image {} ...", self.inner.image_name);
        if !self.image_exists().await {
            log::debug!("Image {} doesn't exist. Pulling.", self.inner.image_name);
            let progress = self.pull();
            // TODO: Use events to change status...
            self.status.set(Status::PullingImage { progress });
        } else {
            log::debug!("Image {} exists. Skip pulling.", self.inner.image_name);
            self.status.set(Status::CleanDangling);
        }
        // Pulling the images if not exists
        Ok(())
    }

    async fn do_clean_dangling(&mut self) -> Result<(), Error> {
        log::debug!("Cheking container {} ...", self.inner.container_name);
        let state = self.container_state().await;
        match state {
            ContainerState::Running => {
                log::debug!("Container {} is running. Terminating it.", self.inner.container_name);
                self.try_kill_container().await?;
                self.status.set(Status::WaitContainerKilled);
            },
            ContainerState::NotRunning => {
                log::debug!("Container {} is not running. Removing it.", self.inner.container_name);
                self.try_remove_container().await?;
                self.status.set(Status::WaitContainerRemoved);
            },
            ContainerState::NotFound => {
                log::debug!("Container {} doesn't exist.", self.inner.container_name);
                self.status.set(Status::Idle);
            },
        }
        Ok(())
    }

    async fn do_wait_container_killed(&mut self) -> Result<(), Error> {
        // TODO: Wait interval
        Ok(())
    }

    async fn do_wait_container_removed(&mut self) -> Result<(), Error> {
        // TODO: Wait interval
        Ok(())
    }

    async fn do_idle(&mut self) -> Result<(), Error> {
        if self.should_be_active() {
            log::debug!("Preparing a container {} to start...", self.inner.container_name);
            self.status.set(Status::CreateContainer);
        }
        Ok(())
    }

    async fn do_create_container(&mut self) -> Result<(), Error> {
        log::debug!("Trying to create container {} ...", self.inner.container_name);
        self.try_create_container().await?;
        self.status.set(Status::WaitContainerCreated);
        Ok(())
    }

    async fn do_wait_container_created(&mut self) -> Result<(), Error> {
        // TODO: Check timeout
        Ok(())
    }

    async fn do_start_container(&mut self) -> Result<(), Error> {
        self.try_start_container().await?;
        self.status.set(Status::WaitContainerStarted);
        Ok(())
    }

    async fn do_started(&mut self) -> Result<(), Error> {
        // TODO: Spawn `ready to use` worker

        // TODO: Remove the following
        if !self.should_be_active() {
            self.status.set(Status::CleanDangling);
        }
        Ok(())
    }

    async fn do_wait_container_started(&mut self) -> Result<(), Error> {
        Ok(())
    }
}
