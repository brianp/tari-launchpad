use derive_more::Deref;
use tokio::time::Instant;

use crate::task::TaskStatus;

pub struct Fallback<S> {
    pub when: Instant,
    pub next_status: S,
}

#[derive(Deref)]
pub struct SdmStatus<S> {
    name: String,
    #[deref]
    status: S,
    has_work: bool,
    fallback: Option<Fallback<S>>,
}

impl<S: Default> SdmStatus<S> {
    pub fn new(name: String) -> Self {
        Self {
            name,
            status: S::default(),
            has_work: false,
            fallback: None,
        }
    }
}

impl<S> SdmStatus<S> {
    pub fn get(&self) -> &S {
        &self.status
    }

    pub fn has_work(&self) -> bool {
        self.has_work
    }

    pub fn reset_has_work_flag(&mut self) {
        self.has_work = false;
    }
}

impl<S: TaskStatus> SdmStatus<S> {
    pub fn check_fallback(&mut self) {
        if let Some(fallback) = self.fallback.as_ref() {
            let now = Instant::now();
            if fallback.when < now {
                let fallback = self.fallback.take().unwrap();
                self.set(fallback.next_status);
            }
        }
    }

    pub fn set(&mut self, status: S) {
        log::debug!("Set the new status !{}::status={:?}", self.name, self.status);
        self.status = status;
        self.has_work = true;
        self.fallback = None;
    }

    pub fn set_fallback(&mut self, fallback: Fallback<S>) {
        self.fallback = Some(fallback);
    }
}
