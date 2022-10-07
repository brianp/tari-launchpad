use anyhow::Error;
use yew::{html, Html};

use crate::{
    scenes::{counter::Counter, main_scene::MainScene},
    states::local_state::{LocalState, LocalStateDelta},
    widget::{Connected, Context, FromDelta, Pod, Widget},
};

pub struct App {
    local_state: Connected<LocalState>,
}

#[derive(Clone)]
pub enum Msg {
    Event,
}

impl FromDelta<LocalState> for Msg {
    fn from_delta(delta: &LocalStateDelta) -> Option<Self> {
        None
    }
}

impl Widget for App {
    type Msg = Msg;

    fn create(ctx: &mut Context<Self>) -> Self {
        Self {
            local_state: ctx.connect::<LocalState>(),
        }
    }

    fn on_event(&mut self, msg: Self::Msg, ctx: &mut Context<Self>) -> Result<(), Error> {
        match msg {
            Msg::Event => {
                // self.local_state.update(LocalStateDelta::Add);
            },
        }
        ctx.redraw();
        Ok(())
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <Pod<MainScene> />
            /*
            <div>
                <Pod<Counter> />
                <p onclick={ctx.event(Msg::Event)}>{ "Tari Launchpad" }</p>
            </div>
            */
        }
    }
}
