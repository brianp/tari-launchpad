mod task;

use std::fmt;

pub(crate) use task::NetworkTask;

use crate::config::ManagedProtocol;

pub trait ManagedNetwork: fmt::Debug + Send + 'static {
    type Config: ManagedProtocol;

    fn reconfigure(&mut self, config: Option<&Self::Config>) -> bool {
        // Start if config exists
        config.is_some()
    }

    // TODO: Move to `Hierarchy`
    fn network_name(&self) -> &str;
}
