use std::thread::LocalKey;

use derive_more::Deref;
use yew::{html::Scope, Callback, Context as YewContext};

use super::{
    pod::{Msg, Pod},
    subscribe::{Connected, FromDelta, SharedState, State},
    widget::Widget,
};

/// The scope that extends [`Scope`] to have extra
/// methods to construct special callbacks for the `Widget`.
#[derive(Deref)]
pub struct PodScope<W: Widget> {
    scope: Scope<Pod<W>>,
}

impl<W: Widget> Clone for PodScope<W> {
    fn clone(&self) -> Self {
        Self {
            scope: self.scope.clone(),
        }
    }
}

impl<W: Widget> Context<W> {
    pub fn connect<T>(&mut self) -> Connected<T>
    where
        T: State,
        W::Msg: FromDelta<T>,
    {
        T::INSTANCE.register(self)
    }

    pub fn should_redraw(&self) -> bool {
        self.redraw
    }

    pub fn no_redraw(&mut self) {
        self.redraw = false;
    }

    pub fn redraw(&mut self) {
        self.redraw = true;
    }

    pub fn link(&self) -> &PodScope<W> {
        &self.scope
    }
}

impl<W: Widget> PodScope<W> {
    pub fn callback<F, IN>(&self, f: F) -> Callback<IN>
    where
        F: Fn(IN) -> W::Msg,
        F: 'static,
    {
        let f = move |input| Msg::WidgetMsg(f(input));
        self.scope.callback(f)
    }

    pub fn event<IN>(&self, msg: W::Msg) -> Callback<IN>
    where W::Msg: Clone {
        self.scope.callback(move |_: IN| Msg::WidgetMsg(msg.clone()))
    }
}

#[derive(Deref)]
pub struct Context<W: Widget> {
    redraw: bool,
    #[deref]
    scope: PodScope<W>,
}

impl<W: Widget> Context<W> {
    pub(super) fn new(ctx: &YewContext<Pod<W>>) -> Self {
        let scope = ctx.link().clone();
        Self {
            redraw: false,
            scope: PodScope { scope },
        }
    }
}
