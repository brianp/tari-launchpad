use anyhow::Error;
use strum::IntoEnumIterator;
use yew::{classes, html, Html};

use crate::{
    states::local_state::{LocalState, LocalStateDelta, Scene},
    widget::{AcceptAll, Connected, Context, FromDelta, Widget},
};

pub struct MainSceneHeader {
    local_state: Connected<LocalState>,
}

impl Widget for MainSceneHeader {
    type Msg = AcceptAll;

    fn create(ctx: &mut Context<Self>) -> Self {
        Self {
            local_state: ctx.connect(),
        }
    }

    fn on_event(&mut self, msg: Self::Msg, ctx: &mut Context<Self>) -> Result<(), Error> {
        log::info!("Event received: {:?}", msg);
        ctx.redraw();
        Ok(())
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let local_state = self.local_state.get();
        html! {
            <div class="main_scene-header">
                { for Scene::iter().map(|item| self.render_menu_item(item, ctx)) }
            </div>
        }
    }
}

impl MainSceneHeader {
    fn render_menu_item(&self, scene: Scene, ctx: &Context<Self>) -> Html {
        let event = LocalStateDelta::SetScene(scene.clone());
        let onclick = self.local_state.event(event);
        let selected = (self.local_state.get().scene == scene).then(|| "selected");
        html! {
            <div class={classes!("menu-item", selected)} {onclick}>{ scene }</div>
        }
    }
}
