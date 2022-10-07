pub mod config;

use std::collections::{HashMap, VecDeque};

use config::LaunchpadConfig;
use serde::{Deserialize, Serialize};

/// An action sent from UI to the backend.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Incoming {
    Action(LaunchpadAction),
    Start,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LaunchpadAction {
    Connect,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LaunchpadDelta {
    UpdateConfig(LaunchpadConfig),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LaunchpadState {
    pub config: LaunchpadConfig,
    pub containers: HashMap<String, String>,
}

pub struct ContainerRecord {
    pub logs: VecDeque<String>,
}

/// A message that is sent from the backend to the UI.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Outgoing {
    StateIsReady(LaunchpadState),
    Delta(LaunchpadDelta),
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
