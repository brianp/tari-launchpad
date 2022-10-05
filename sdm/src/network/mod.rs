mod task;

use std::fmt;

pub(crate) use task::NetworkTask;

use crate::config::ManagedProtocol;

pub trait ManagedNetwork: fmt::Debug + Send + 'static {
    type Protocol: ManagedProtocol;

    fn reconfigure(&mut self, config: Option<&<Self::Protocol as ManagedProtocol>::Config>) -> bool {
        // Start if config exists
        config.is_some()
    }

    // TODO: Move to `Hierarchy`
    fn network_name(&self) -> &str;
}
