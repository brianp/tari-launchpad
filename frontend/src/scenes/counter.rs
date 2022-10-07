use anyhow::Error;
use yew::{html, Html};

use crate::{
    states::local_state::{LocalState, LocalStateDelta},
    widget::{Connected, Context, FromDelta, Widget},
};

pub struct Counter {
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

impl Widget for Counter {
    type Msg = Msg;

    fn create(ctx: &mut Context<Self>) -> Self {
        Self {
            local_state: ctx.connect(),
        }
    }

    fn on_event(&mut self, msg: Self::Msg, ctx: &mut Context<Self>) -> Result<(), Error> {
        // crate::api::request();
        ctx.redraw();
        Ok(())
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let local_state = self.local_state.get();
        html! {
            //<div>{ local_state.counter }</div>
        }
    }
}
