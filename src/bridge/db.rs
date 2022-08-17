use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = r"/src/bridge/js/db.js")]
extern "C" {

    pub async fn take_data(root_user: String) -> JsValue;

    pub fn write_data(userId: String, data: String);
}
