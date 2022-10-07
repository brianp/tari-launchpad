use anyhow::Error;
use tari_sdm_launchpad_bus::LaunchpadBus;
use tauri::{App, Manager, Wry};

static REQUESTS: &str = "requests";
static RESPONSES: &str = "responses";

pub fn bus_setup(app: &mut App<Wry>) -> Result<(), Box<dyn std::error::Error>> {
    let handle = app.handle();
    let bus = LaunchpadBus::start()?;

    let in_tx = bus.incoming;
    let _id = app.listen_global(REQUESTS, move |event| {
        if let Some(payload) = event.payload() {
            let res = serde_json::from_str(payload);
            match res {
                Ok(incoming) => {
                    log::trace!("Incoming event: {:?}", incoming);
                    if let Err(err) = in_tx.send(incoming) {
                        log::error!("Can't forward an incoming event: {:?}", err);
                    }
                },
                Err(err) => {
                    log::error!("Can't parse incoming event: {}", err);
                },
            }
        }
    });

    let mut out_rx = bus.outgoing;
    tauri::async_runtime::spawn(async move {
        loop {
            if let Some(event) = out_rx.recv().await {
                handle.emit_all(RESPONSES, event)?;
            } else {
                break;
            }
        }
        Ok::<(), Error>(())
    });

    Ok(())
}
