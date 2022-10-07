use anyhow::Error;
use tari_launchpad_protocol::Incoming;
use yew::{html, Html};

use crate::{
    bus,
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
    Start,
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
            Msg::Start => {
                log::info!("Starting...");
                bus::request(Incoming::Start);
            },
        }
        ctx.redraw();
        Ok(())
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <div onclick={ctx.event(Msg::Start)} >{ "Start" }</div>
                <Pod<MainScene> />
            </div>
            /*
            <div>
                <Pod<Counter> />
                <p onclick={ctx.event(Msg::Event)}>{ "Tari Launchpad" }</p>
            </div>
            */
        }
    }
}
