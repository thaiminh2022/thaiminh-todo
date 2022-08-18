use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = r"/src/bridge/js/generic.js")]
extern "C" {
    pub async fn hello(msg: &str) -> JsValue;

    pub async fn fetch_data() -> JsValue;

    #[wasm_bindgen(js_name = log)]
    pub fn log(msg: &str);

    pub fn log_json(msg: &str);

    pub fn page_reloaded() -> bool;
}
