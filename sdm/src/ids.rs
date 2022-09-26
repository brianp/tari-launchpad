use derive_more::{From, Into};

pub trait ManagedTask {
    fn id() -> TaskId;

    fn deps() -> Vec<TaskId> {
        Vec::default()
    }
}

#[derive(Debug, Clone, From, Into, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct TaskId(String);

impl From<&str> for TaskId {
    fn from(s: &str) -> Self {
        Self(s.into())
    }
}
