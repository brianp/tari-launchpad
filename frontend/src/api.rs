use wasm_bindgen::prelude::{wasm_bindgen, Closure, JsValue};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["__TAURI__", "event"])]
    async fn listen(event: &str, f: &Closure<dyn FnMut(JsValue)>) -> JsValue;
    #[wasm_bindgen(js_namespace = ["__TAURI__", "event"])]
    fn emit(event: &str, object: JsValue) -> JsValue;
}

pub fn request() {
    emit("requests", JsValue::NULL);
}

pub async fn connect_to_bus() {
    let closure = Closure::new(process_response);
    let _unlisten_promise = listen("responses", &closure).await;
    closure.forget();
}

fn process_response(value: JsValue) {
    log::trace!("EVENT: {:?}", value);
}
