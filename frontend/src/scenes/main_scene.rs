use anyhow::Error;
use yew::{html, Html};

use crate::{
    scenes::main_scene_header::MainSceneHeader,
    states::local_state::{LocalState, LocalStateDelta},
    widget::{Connected, Context, FromDelta, Pod, Widget},
};

pub struct MainScene {
    local_state: Connected<LocalState>,
}

#[derive(Clone)]
pub enum Msg {
    Updated,
}

impl FromDelta<LocalState> for Msg {
    fn from_delta(delta: &LocalStateDelta) -> Option<Self> {
        Some(Msg::Updated)
    }
}

impl Widget for MainScene {
    type Msg = Msg;

    fn create(ctx: &mut Context<Self>) -> Self {
        Self {
            local_state: ctx.connect(),
        }
    }

    fn on_event(&mut self, msg: Self::Msg, ctx: &mut Context<Self>) -> Result<(), Error> {
        ctx.redraw();
        Ok(())
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let local_state = self.local_state.get();
        html! {
            <div class="main_scene">
                <div class="main_scene-content">
                    <Pod<MainSceneHeader> />
                </div>
            </div>
        }
    }
}
