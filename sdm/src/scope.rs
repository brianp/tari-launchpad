use std::sync::Arc;

use anyhow::{anyhow, Error};
use bollard::Docker;
use tokio::sync::{broadcast, mpsc};

use crate::{
    config::ManagedProtocol,
    ids::{ManagedTask, TaskId},
    image::{ImageTask, ManagedContainer},
    network::{ManagedNetwork, NetworkTask},
    task::SdmTaskRunner,
    volume::{ManagedVolume, VolumeTask},
};

pub struct Report<C: ManagedProtocol> {
    task_id: TaskId,
    details: ReportDetails<C>,
}

pub enum ReportDetails<C: ManagedProtocol> {
    Started,
    Progress,
    Ready,
    Closed,
    Extras(C::Outer),
}

pub struct SdmScope<C: ManagedProtocol> {
    scope: String,
    docker: Docker,
    reporter: mpsc::UnboundedReceiver<Report<C>>,
    report_sender: mpsc::UnboundedSender<Report<C>>,
    sender: broadcast::Sender<ControlEvent<C>>,
}

// TODO: Move to the `task` mod?
#[derive(Debug)]
pub enum ControlEvent<C: ManagedProtocol> {
    SetConfig(Option<Arc<C::Config>>),
    ResourceReady {
        task_id: TaskId,
        /// id or name in the docker
        name: String,
    },
    ResourceClosed {
        task_id: TaskId,
    },
    InnerEvent(C::Inner),
}

impl<C: ManagedProtocol> Clone for ControlEvent<C> {
    fn clone(&self) -> Self {
        match self {
            Self::SetConfig(config) => Self::SetConfig(config.clone()),
            Self::ResourceReady { task_id, name } => Self::ResourceReady {
                task_id: task_id.clone(),
                name: name.clone(),
            },
            Self::ResourceClosed { task_id } => Self::ResourceClosed {
                task_id: task_id.clone(),
            },
            Self::InnerEvent(inner) => Self::InnerEvent(inner.clone()),
        }
    }
}

impl<C: ManagedProtocol> SdmScope<C> {
    pub fn connect(scope: &str) -> Result<Self, Error> {
        let docker = Docker::connect_with_local_defaults()?;
        // TODO: Use `rx` later to control entries
        let (req_tx, _req_rx) = broadcast::channel(16);
        let (rep_tx, rep_rx) = mpsc::unbounded_channel();
        Ok(Self {
            scope: scope.to_string(),
            docker,
            reporter: rep_rx,
            report_sender: rep_tx,
            sender: req_tx,
        })
    }

    pub async fn add_image<I>(&mut self, entry: I) -> Result<(), Error>
    where I: ManagedContainer<Protocol = C> + ManagedTask {
        // TODO: DRY!
        let entry = Box::new(entry);
        let inner = ImageTask::new(&self.scope, entry);
        let runner = SdmTaskRunner::new::<I>(
            self.sender.clone(),
            self.report_sender.clone(),
            inner,
            self.docker.clone(),
        );
        tokio::spawn(runner.entrypoint());
        Ok(())
    }

    pub async fn add_network<N>(&mut self, entry: N) -> Result<(), Error>
    where N: ManagedNetwork<Protocol = C> + ManagedTask {
        // TODO: DRY!
        let entry = Box::new(entry);
        let inner = NetworkTask::new(&self.scope, entry);
        let runner = SdmTaskRunner::new::<N>(
            self.sender.clone(),
            self.report_sender.clone(),
            inner,
            self.docker.clone(),
        );
        tokio::spawn(runner.entrypoint());
        Ok(())
    }

    pub async fn add_volume<V>(&mut self, entry: V) -> Result<(), Error>
    where V: ManagedVolume<Protocol = C> + ManagedTask {
        // TODO: DRY!
        let entry = Box::new(entry);
        let inner = VolumeTask::new(&self.scope, entry);
        let runner = SdmTaskRunner::new::<V>(
            self.sender.clone(),
            self.report_sender.clone(),
            inner,
            self.docker.clone(),
        );
        tokio::spawn(runner.entrypoint());
        Ok(())
    }

    pub async fn set_config(&mut self, config: Option<C::Config>) -> Result<(), Error> {
        let config = config.map(Arc::new);
        let req = ControlEvent::SetConfig(config);
        self.send(req)
    }

    fn send(&mut self, req: ControlEvent<C>) -> Result<(), Error> {
        self.sender
            .send(req)
            .map(drop)
            .map_err(|req| anyhow!("Can't send a request: {:?}", req))
    }

    pub fn stop(&self) {}
}
