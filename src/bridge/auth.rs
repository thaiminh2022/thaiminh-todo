use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = r"/src/bridge/js/auth.js")]
extern "C" {

    #[wasm_bindgen(catch)]
    pub async fn login_email_password(email: String, password: String) -> Result<(), JsValue>;

    #[wasm_bindgen(catch)]
    pub fn get_user_uid() -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch)]
    pub async fn log_out() -> Result<(), JsValue>;

    #[wasm_bindgen(catch)]
    pub async fn login_google_accout() -> Result<(), JsValue>;

    #[wasm_bindgen(catch)]
    pub fn get_user_display_name() -> Result<JsValue, JsValue>;

}
