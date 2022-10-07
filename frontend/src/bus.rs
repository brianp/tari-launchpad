use anyhow::Error;
use tari_launchpad_protocol::Incoming;
use wasm_bindgen::prelude::{wasm_bindgen, Closure, JsValue};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["__TAURI__", "event"])]
    async fn listen(event: &str, f: &Closure<dyn FnMut(JsValue)>) -> JsValue;
    #[wasm_bindgen(js_namespace = ["__TAURI__", "event"])]
    fn emit(event: &str, object: JsValue) -> JsValue;
}

pub fn request(incoming: Incoming) {
    log::info!("Sending: {:?}", incoming);
    if let Err(err) = request_impl(incoming) {
        log::error!("Can't serialize an incoming event: {}", err);
    }
}

fn request_impl(incoming: Incoming) -> Result<(), Error> {
    let value = serde_json::to_string(&incoming)?;
    let js_value = JsValue::from_str(&value);
    emit("requests", js_value);
    Ok(())
}

pub async fn connect_to_bus() {
    let closure = Closure::new(process_response);
    let _unlisten_promise = listen("responses", &closure).await;
    closure.forget();
}

fn process_response(value: JsValue) {
    log::trace!("EVENT: {:?}", value);
}
