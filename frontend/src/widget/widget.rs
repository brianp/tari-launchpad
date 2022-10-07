use anyhow::Error;
use yew::Html;

use super::context::Context;

pub trait Widget: Sized + 'static {
    type Msg;

    fn create(ctx: &mut Context<Self>) -> Self;

    fn initialize(&mut self, _ctx: &mut Context<Self>) -> Result<(), Error> {
        Ok(())
    }

    fn on_event(&mut self, _msg: Self::Msg, ctx: &mut Context<Self>) -> Result<(), Error> {
        ctx.redraw();
        Ok(())
    }

    fn view(&self, _ctx: &Context<Self>) -> Html;
}
