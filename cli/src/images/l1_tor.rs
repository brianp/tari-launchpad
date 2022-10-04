use async_trait::async_trait;
use regex::Regex;
use tari_sdm::{
    ids::{ManagedTask, TaskId},
    image::{
        checker::{CheckerContext, CheckerEvent, ContainerChecker},
        Args,
        Envs,
        ManagedContainer,
        Networks,
    },
};

use super::DEFAULT_REGISTRY;
use crate::{
    config::{ConnectionSettings, LaunchpadConfig},
    networks::LocalNet,
    volumes::SharedVolume,
};

#[derive(Debug, Default)]
pub struct Tor {
    settings: Option<ConnectionSettings>,
}

impl ManagedTask for Tor {
    fn id() -> TaskId {
        "Tor".into()
    }

    fn deps() -> Vec<TaskId> {
        vec![LocalNet::id(), SharedVolume::id()]
    }
}

impl ManagedContainer for Tor {
    type Config = LaunchpadConfig;

    fn registry(&self) -> &str {
        DEFAULT_REGISTRY
    }

    fn image_name(&self) -> &str {
        "tor"
    }

    fn reconfigure(&mut self, config: Option<&Self::Config>) -> bool {
        self.settings = config.map(ConnectionSettings::from);
        self.settings.is_some()
    }

    fn checker(&mut self) -> Box<dyn ContainerChecker> {
        Box::new(TorChecker::new())
    }

    fn args(&self, args: &mut Args) {
        args.set_pair("--SocksPort", "0.0.0.0:9050");
        args.set_pair("--ControlPort", "0.0.0.0:9051");
        args.set_pair("--CookieAuthentication", 0);
        args.set_pair("--ClientOnly", 1);
        args.set_pair("--ClientUseIPv6", 1);
        if let Some(settings) = self.settings.as_ref() {
            args.set_pair("--HashedControlPassword", settings.tor_password.reveal());
        }
        args.flag("--allow-missing-torrc");
    }

    fn envs(&self, envs: &mut Envs) {
        if let Some(settings) = self.settings.as_ref() {
            settings.apply(envs);
        }
    }

    fn networks(&self, networks: &mut Networks) {
        networks.add(LocalNet::id());
    }
}

pub struct TorChecker {
    re: Regex,
}

impl TorChecker {
    fn new() -> Self {
        let re = Regex::new(r"Bootstrapped\s+(?P<pct>\d+)%").unwrap();
        Self { re }
    }
}

#[async_trait]
impl ContainerChecker for TorChecker {
    // TODO: Add result here?
    async fn on_log_event(&mut self, record: String, ctx: &mut CheckerContext) {
        if let Some(caps) = self.re.captures(&record) {
            if let Some(value) = caps.name("pct") {
                if let Ok(value) = value.as_str().parse() as Result<i32, _> {
                    ctx.send(CheckerEvent::Progress(value as u8)).ok();
                    if value == 100 {
                        ctx.send(CheckerEvent::Ready).ok();
                    }
                }
            }
        }
    }
}