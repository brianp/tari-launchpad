use std::{collections::HashMap, fmt, ops::Deref};

use anyhow::Error;
use async_trait::async_trait;
use bollard::Docker;
use derive_more::{Deref, DerefMut};
use futures::StreamExt;
use tokio::{
    select,
    sync::{broadcast, mpsc},
    time::{sleep, Duration, Instant},
};
use tokio_stream::wrappers::{BroadcastStream, UnboundedReceiverStream};

use crate::{
    config::ManagedProtocol,
    ids::{ManagedTask, TaskId},
    scope::ControlEvent,
    status::SdmStatus,
};

pub trait TaskStatus: fmt::Debug + Default + Send {
    fn is_ready(&self) -> bool {
        false
    }
}

#[async_trait]
pub trait RunnableTask: Sized + Send + 'static {
    type Protocol: ManagedProtocol;
    type Status: TaskStatus;
    type Event: TaskEvent;

    fn name(&self) -> &str;
}

#[async_trait]
pub trait RunnableContext<T: RunnableTask> {
    /// Subscribe to events here
    async fn initialize(&mut self);
    fn reconfigure(&mut self, config: Option<&<T::Protocol as ManagedProtocol>::Config>) -> bool;
    fn process_event(&mut self, event: T::Event) -> Result<(), Error>;
    async fn update(&mut self) -> Result<(), Error>;
}

#[derive(Deref, DerefMut)]
pub struct TaskContext<T: RunnableTask> {
    /// Filled by a dependencies controller
    dependencies_ready: bool,
    resources_map: HashMap<TaskId, String>,
    /// Depends on the config
    should_start: bool,
    pub status: SdmStatus<T::Status>,
    pub sender: mpsc::UnboundedSender<T::Event>,
    pub driver: Docker,

    #[deref]
    #[deref_mut]
    pub inner: T,
}

impl<T: RunnableTask> TaskContext<T> {
    pub fn should_be_active(&self) -> bool {
        self.should_start && self.dependencies_ready
    }

    pub fn resource(&self, id: &TaskId) -> Option<&str> {
        self.resources_map.get(id).map(String::as_ref)
    }
}

pub struct SdmTaskRunner<R: RunnableTask> {
    task_id: TaskId,
    events_receiver: Option<mpsc::UnboundedReceiver<R::Event>>,
    requests_receiver: Option<broadcast::Receiver<ControlEvent<R::Protocol>>>,
    requests_sender: broadcast::Sender<ControlEvent<R::Protocol>>,

    context: TaskContext<R>,
    next_update: Instant,
    /// Waits when these dependencies started.
    dependencies: HashMap<TaskId, bool>,
    ready_to_use: bool,
}

impl<R: RunnableTask> SdmTaskRunner<R>
where TaskContext<R>: RunnableContext<R>
{
    pub fn new<M: ManagedTask>(sender: broadcast::Sender<ControlEvent<R::Protocol>>, inner: R, docker: Docker) -> Self {
        let (event_tx, event_rx) = mpsc::unbounded_channel();
        let context = TaskContext {
            dependencies_ready: false,
            resources_map: HashMap::new(),
            should_start: false,
            status: SdmStatus::new(inner.name().to_string()),
            sender: event_tx,
            driver: docker,
            inner,
        };
        // It subscribed here to avoid the gap if
        // that will subscribe in the routine.
        let req_rx = sender.subscribe();
        let dependencies = M::deps().into_iter().map(|id| (id, false)).collect();
        Self {
            task_id: M::id(),
            events_receiver: Some(event_rx),
            requests_receiver: Some(req_rx),
            requests_sender: sender,
            context,
            next_update: Instant::now(),
            dependencies,
            ready_to_use: false,
        }
    }

    pub async fn entrypoint(mut self) {
        self.check_dependencies();
        self.initialize().await;
        let interval = Duration::from_millis(1_000);
        let events_receiver = self.events_receiver.take().unwrap();
        let mut events = UnboundedReceiverStream::new(events_receiver);
        let requests_receiver = self.requests_receiver.take().unwrap();
        let mut requests = BroadcastStream::new(requests_receiver);
        loop {
            select! {
                _ = sleep(interval) => {
                    // log::trace!("Checking the scope by interval");
                }
                event = events.next() => {
                    if let Some(event) = event {
                        self.process_event(event);
                    } else {
                        log::info!("Events stream closed");
                        break;
                    }
                }
                req = requests.next() => {
                    if let Some(Ok(req)) = req {
                        self.process_request(req);
                    } else {
                        log::info!("Requests stream closed");
                        break;
                    }
                }
            }
            self.update().await;
            self.notify_dependants();
        }
    }

    fn notify_dependants(&mut self) {
        if self.context.status.is_ready() {
            if !self.ready_to_use {
                self.ready_to_use = true;
                // Notifies dependants about the entity is ready to use
                let task_id = self.task_id.clone();
                let name = self.context.name().to_owned();
                let event = ControlEvent::ResourceReady { task_id, name };
                self.broadcast(event);
            }
        } else {
            if self.ready_to_use {
                self.ready_to_use = false;
                // Notifies dependants about the entity is not ready anymore
                let task_id = self.task_id.clone();
                let event = ControlEvent::ResourceClosed { task_id };
                self.broadcast(event);
            }
        }
    }

    fn broadcast(&mut self, event: ControlEvent<R::Protocol>) {
        if let Err(err) = self.requests_sender.send(event) {
            log::error!("Can't brodcast event: {:?}", err);
        }
    }

    fn process_request(&mut self, req: ControlEvent<R::Protocol>) {
        match req {
            ControlEvent::SetConfig(config) => {
                let config = config.as_ref().map(Deref::deref);
                self.reconfigure(config);
            },
            ControlEvent::ResourceReady { task_id, name } => {
                if let Some(flag) = self.dependencies.get_mut(&task_id) {
                    *flag = true;
                    self.context.resources_map.insert(task_id, name);
                    // Check dependencies if any flag changed
                    self.check_dependencies();
                }
            },
            ControlEvent::ResourceClosed { task_id } => {
                if let Some(flag) = self.dependencies.get_mut(&task_id) {
                    *flag = false;
                    self.context.resources_map.remove(&task_id);
                    // Check dependencies if any flag changed
                    self.check_dependencies();
                }
            },
        }
    }

    fn check_dependencies(&mut self) {
        // If the set is empty `all` returs `true`.
        self.context.dependencies_ready = self.dependencies.values().all(|ready| *ready);
    }

    pub async fn initialize(&mut self) {
        self.context.initialize().await;
    }

    pub fn reconfigure(&mut self, config: Option<&<R::Protocol as ManagedProtocol>::Config>) {
        let active = self.context.reconfigure(config);
        self.context.should_start = active;
    }

    pub fn process_event(&mut self, event: R::Event) {
        println!("!{:<20} [event]  = {:?}", self.context.name(), event);
        if let Err(err) = self.context.process_event(event) {
            log::error!("Event processing error: {}", err);
        }
    }

    pub async fn update(&mut self) {
        println!(
            "!{:<20} [update] = {:?}",
            self.context.name(),
            self.context.status.get()
        );
        loop {
            let now = Instant::now();
            if self.next_update > now {
                continue;
            }
            self.context.status.check_fallback();
            self.context.status.reset_has_work_flag();
            if let Err(err) = self.context.update().await {
                log::error!("Update error: {}", err);
                self.next_update = now + Duration::from_secs(5);
                // TODO: Set a fallback? and reset it when succeed?
                break;
            }
            if !self.context.status.has_work() {
                break;
            }
        }
    }
}

pub trait TaskEvent: fmt::Debug + Send {}
