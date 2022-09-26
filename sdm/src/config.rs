use std::fmt;

pub trait ManagedConfig: fmt::Debug + Sync + Send + 'static {}
