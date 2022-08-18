use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = r"/src/bridge/js/storage.js")]
extern "C" {

    #[wasm_bindgen(catch)]
    pub fn set_item_storage(key: String, item: String) -> Result<(), JsValue>;

    #[wasm_bindgen(catch)]
    pub fn get_item_storage(key: String) -> Result<String, JsValue>;

    pub fn clear_item_storage();
}
