use tari_sdm_launchpad::config::LaunchpadConfig;

pub struct LaunchpadState {
    pub config: LaunchpadConfig,
}

impl LaunchpadState {
    pub fn apply(&mut self, delta: LaunchpadDelta) {
        use LaunchpadDelta::*;
        match delta {
            UpdateConfig(config) => {
                self.config = config;
            },
        }
    }
}

pub enum LaunchpadDelta {
    UpdateConfig(LaunchpadConfig),
}
