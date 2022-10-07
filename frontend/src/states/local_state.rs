use strum::{Display, EnumIter};

use crate::widget::{SharedState, State};

#[derive(EnumIter, Display, Clone, Debug, PartialEq, Eq)]
pub enum Scene {
    Mining,
    #[strum(to_string = "Base Node")]
    BaseNode,
    Wallet,
}

impl Default for Scene {
    fn default() -> Self {
        Self::BaseNode
    }
}

#[derive(Default, Debug)]
pub struct LocalState {
    pub scene: Scene,
}

impl State for LocalState {
    type Delta = LocalStateDelta;

    const INSTANCE: SharedState<Self> = SharedState::new();

    fn apply(&mut self, delta: Self::Delta) {
        match delta {
            LocalStateDelta::SetScene(scene) => {
                self.scene = scene;
            },
        }
        log::debug!("Local state updated: {:?}", self);
    }
}

#[derive(Clone)]
pub enum LocalStateDelta {
    SetScene(Scene),
}
