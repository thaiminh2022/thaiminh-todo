use std::{collections::HashMap, fmt::Error};

use gloo_timers::future::TimeoutFuture;
use yew::prelude::*;
use yew_router::prelude::*;

mod bridge;
mod comps;
mod custom;
mod data;

use bridge::*;
use comps::{Counter, LoginPage, Model, NotFound};
use custom::CustomButton;
use data::{Route, TodoData, UserData};

fn switch(routes: &Route) -> Html {
    match routes {
        Route::Login => html! {<LoginPage />},
        Route::Home => html! {<LoginPage/>},
        Route::Todo => html! {<Model/> },
        Route::NotFound => html! {<NotFound/>},
        Route::Counter => html! {<Counter/>},
    }
}

pub struct App {
    user: UserData,

    side_bar_active: bool,
}

pub enum Msg {
    FlipSideBarActive,
    FlipDarkModeActive,

    UpdateFullData(UserData),
    UpdateDisplayName(String),

    FetchSettingFromDatabase,
    WriteSettingToDatabase,

    Nothing,
    Refresh,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            user: UserData::new(String::from("User-69420"), true),
            side_bar_active: false,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();

        let link_text: HashMap<String, String> = HashMap::from([
            ("/todo".to_string(), "Todo".to_string()),
            ("/counter".to_string(), "Counter".to_string()),
            ("/login".to_string(), "Login".to_string()),
        ]);

        let nav_items = |(_, (address, text)): (usize, (&String, &String))| -> Html {
            html! {
                <li>
                    <a href={address.to_string()}>
                        <CustomButton text = {text.to_string()} onclick = {link.callback(|_| Msg::Nothing)} />
                    </a>
                </li>
            }
        };
        html! {
            <div class= {
                if self.user.prefer_dark_mode == true{
                    "wrapper-all dark-mode"
                }
                else{
                    "wrapper-all"
                }


            }>
                <header class ="nav-header">
                    <div class="logo">
                        <h1> {"Thaiminh2022"} </h1>
                        <i class={
                            if self.user.prefer_dark_mode == true{
                                "bx bxs-moon"
                            }
                            else{
                                "bx bxs-sun"
                            }
                        }

                        onclick = {link.callback(|_| Msg::FlipDarkModeActive)}

                         />

                    </div>

                    <nav>
                        <ul class="nav-links">
                            {for link_text.iter().enumerate().map(nav_items)}
                        </ul>
                    </nav>
                    <h1 class="user-name">{self.user.display_name.clone()}</h1>
                    <a href="#" class="menu"><button onclick= {link.callback(|_| Msg::FlipSideBarActive)}><i class= "bx bx-menu" /></button></a>
                </header>


                <div class="nav-side" hidden = {!self.side_bar_active}>
                    <nav>
                        <ul class="nav-links">
                            {for link_text.iter().enumerate().map(nav_items)}
                        </ul>
                    </nav>
                </div>


                <BrowserRouter>
                    <Switch<Route> render={Switch::render(switch)} />
                </BrowserRouter>
            </div>
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let link = ctx.link();

        match msg {
            Msg::FlipSideBarActive => {
                // flip the side bar
                self.side_bar_active = !self.side_bar_active;
            }
            Msg::FlipDarkModeActive => {
                // flip the dark mode
                self.user.prefer_dark_mode = !self.user.prefer_dark_mode;

                // Save to data base
                link.send_message(Msg::WriteSettingToDatabase);
            }
            Msg::UpdateDisplayName(s) => {
                // Update display name
                self.user.display_name = s;
            }
            Msg::UpdateFullData(s) => {
                self.user = s.clone();
            }

            Msg::FetchSettingFromDatabase => {
                link.send_future(App::fetch_database());
            }
            Msg::WriteSettingToDatabase => {
                // Jolder
                let data_json = self.user.to_json();
                link.send_message(App::write_database(data_json));
                clear_item_storage();
            }
            Msg::Nothing => return false,
            Msg::Refresh => return true,
        }

        true
    }
    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render {
            let link = ctx.link().clone();

            // let result = App::from_local_storage();
            // match result {
            //     Ok(s) => {
            //         self.user = s.clone();
            //         ctx.link().send_message(Msg::Refresh);
            //         if s.display_name.trim() != "User-69420".to_string().trim() {
            //             return;
            //         }
            //     }
            //     Err(code) => log(&code.to_string()),
            // }

            wasm_bindgen_futures::spawn_local(async move {
                TimeoutFuture::new(500).await;
                // Do something here after the one second timeout is up!
                link.send_message(App::fetch_database().await);
                link.send_message(App::update_display_name().await);
            })
        }
    }
}

impl App {
    pub async fn update_display_name() -> Msg {
        let result = get_user_display_name();

        let final_string = match result {
            Ok(s) => s.as_string().expect("None value"),
            Err(e) => String::from("User-69420"),
        };

        log(&final_string);
        return Msg::UpdateDisplayName(final_string);
    }

    pub async fn fetch_database() -> Msg {
        let uid_result = get_user_uid();

        let uid = match uid_result {
            Ok(v) => v.as_string().unwrap_or("user_1".to_string()),
            Err(_) => "user_1".to_string(),
        };

        let data: UserData = take_data(uid, "/setting".to_string())
            .await
            .into_serde()
            .unwrap_or_default();
        log("fetched");

        return Msg::UpdateFullData(data);
    }

    pub fn write_database(data: String) -> Msg {
        let uid_result = get_user_uid();

        let uid = match uid_result {
            Ok(v) => v.as_string().unwrap_or("user_1".to_string()),
            Err(_) => "user_1".to_string(),
        };

        write_data(uid, "/setting".to_string(), data);
        return Msg::Refresh;
    }

    pub fn from_local_storage() -> Result<UserData, i32> {
        let data_local_result = get_item_storage("setting".to_string());

        let data_local = match data_local_result {
            Ok(s) => s,
            Err(_) => {
                return Err(0);
            }
        };

        let data_final: UserData = match serde_json::from_str(&data_local) {
            Ok(d) => d,
            Err(_) => {
                return Err(1);
            }
        };

        return Ok(data_final);
    }
}

fn main() {
    // The main file is for creating a nav bar
    yew::start_app::<App>();
}
