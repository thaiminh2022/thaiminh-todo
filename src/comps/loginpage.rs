use web_sys::HtmlInputElement;
use yew::prelude::*;

use super::super::bridge::{get_user_uid, log, log_out, login_email_password, login_google_accout};
use super::super::custom::CustomButton;

pub struct LoginPage {
    login_email: String,
    login_password: String,

    error_message: String,
}

pub enum Msg {
    LoginEmailPasswordAsync,
    LoginGoogleAsync,
    LogOutAsync,

    UpdateLogin(String, String),
    UpdateMessage(String),

    Nothing,

    Log(String),
    LogUID,
}

impl Component for LoginPage {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            login_email: "".to_string(),
            login_password: "".to_string(),

            error_message: "".to_string(),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();

        let on_login_email_change = link.batch_callback(move |e: Event| {
            let input = e.target_dyn_into::<HtmlInputElement>();

            input.map(|input| Msg::UpdateLogin(input.value(), "".to_string()))
        });
        let on_login_password_change = link.batch_callback(move |e: Event| {
            let input = e.target_dyn_into::<HtmlInputElement>();

            input.map(|input| Msg::UpdateLogin("".to_string(), input.value()))
        });

        html! {
            <div class="lp">
                <div class="header">
                    <h1 class="logo">{"Thaiminh2022"}</h1>
                    <h2 class="login-msg">{"Login now"}</h2>
                </div>

                <div class="log-wrapper">
                    <div class="login">
                        <input type="text" placeholder = "Input your email"  class="input-email" onchange ={on_login_email_change} />
                        <input type="password" placeholder= "Input your password" class="input-password" onchange = {on_login_password_change}/>
                        <button class ="forgot-password">{"Forgot password?"}</button>
                    </div>
                    <p class="login-error">{self.error_message.clone()}</p>
                </div>
                <div class="btns">
                    <CustomButton onclick = {link.callback(|_| Msg::LoginEmailPasswordAsync)} text = "Login"/>
                    <CustomButton onclick = {link.callback(|_| Msg::LogOutAsync)} text = "LogOut"/>
                    <CustomButton onclick = {link.callback(|_| Msg::LogUID)} text = "LogUID"/>
                </div>
                <div class="extra">
                    <p class="no-account">{"No account? Click here to register"}</p>

                    <div class="extra-login">
                        <h2 class ="login-with">{"Or login with"}</h2>
                        <div class="extra-btns">
                            <button class="google extra-btn" onclick = {link.callback(|_| Msg::LoginGoogleAsync)}>
                                <i class="bx bxl-google extra-btn-icon"/>
                            </button>
                            <button class="facebook extra-btn">
                                <i class="bx bxl-facebook-circle extra-btn-icon"></i>
                            </button>
                            <button class="github extra-btn">
                                <i class="bx bxl-github extra-btn-icon" />
                            </button>
                        </div>


                    </div>
                </div>
            </div>
        }
    }
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let link = ctx.link();

        match msg {
            Msg::LoginEmailPasswordAsync => {
                link.send_future(LoginPage::login_emailpassword(
                    self.login_email.clone(),
                    self.login_password.clone(),
                ));
            }

            Msg::LoginGoogleAsync => {
                // comments for holder
                log("Hello Google!");
                let link = link.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let result = login_google_accout().await;

                    match result {
                        Ok(_) => {
                            link.send_message(Msg::UpdateMessage("Login OK! 👌🏻".to_string()))
                        }
                        Err(_) => link.send_message(Msg::UpdateMessage(
                            "Login failed, try again".to_string(),
                        )),
                    };
                });
            }
            Msg::LogOutAsync => {
                let link = link.clone();

                wasm_bindgen_futures::spawn_local(async move {
                    let result = log_out().await;

                    match result {
                        Ok(_) => {
                            link.send_message(Msg::UpdateMessage("Logged out OK!".to_string()))
                        }
                        Err(_) => link.send_message(Msg::UpdateMessage(
                            "Cannot log out, did u login?".to_string(),
                        )),
                    }
                })
            }
            Msg::Nothing => {}
            Msg::Log(s) => {
                log(&s);
            }
            Msg::UpdateLogin(m, p) => {
                let mut final_data = LoginPage::new(&m, &p);

                if m.trim().is_empty() {
                    final_data.login_email = self.login_email.clone();
                } else if p.trim().is_empty() {
                    final_data.login_password = self.login_password.clone();
                }

                self.login_email = final_data.login_email;
                self.login_password = final_data.login_password;
            }
            Msg::UpdateMessage(s) => {
                self.error_message = s;
            }
            Msg::LogUID => {
                link.send_future(LoginPage::get_uid());
            }
        }

        true
    }
}

impl LoginPage {
    async fn login_emailpassword(email: String, password: String) -> Msg {
        let result = login_email_password(email, password).await;

        match result {
            Ok(_) => return Msg::UpdateMessage("Logged In!".to_string()),
            Err(_) => {
                return Msg::UpdateMessage(
                    "Cannot login, check your email and password".to_string(),
                )
            }
        };
    }
    async fn get_uid() -> Msg {
        let data = get_user_uid().await;

        match data {
            Ok(s) => return Msg::Log(s.as_string().unwrap_or("user_1".to_string())),
            Err(_) => return Msg::UpdateMessage("Cannot log your UID, did you login?".to_string()),
        };
    }

    fn new(login_email: &str, login_password: &str) -> LoginPage {
        LoginPage {
            login_email: login_email.to_string(),
            login_password: login_password.to_string(),

            error_message: "".to_string(),
        }
    }
}
