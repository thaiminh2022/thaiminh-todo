use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = r"/src/bridge/js/db.js")]
extern "C" {

    pub async fn take_data(root_user: String, path: String) -> JsValue;

    pub fn write_data(userId: String, path: String, data: String);
}
