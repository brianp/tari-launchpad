mod task;

use std::fmt;

pub(crate) use task::NetworkTask;

use crate::config::ManagedConfig;

pub trait ManagedNetwork: fmt::Debug + Send + 'static {
    type Config: ManagedConfig;

    fn reconfigure(&mut self, config: Option<&Self::Config>) -> bool {
        // Start if config exists
        config.is_some()
    }

    // TODO: Move to `Hierarchy`
    fn network_name(&self) -> &str;
}
