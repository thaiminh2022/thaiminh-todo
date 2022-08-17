use std::collections::HashMap;

use yew::prelude::*;
use yew_router::prelude::*;

mod bridge;
mod comps;
mod custom;
mod data;

use comps::{Counter, LoginPage, Model, NotFound};
use custom::CustomButton;
use data::Route;

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
    side_bar_active: bool,
    dark_mode_active: bool,
}

pub enum Msg {
    FlipSideBarActive,
    FlipDarkModeActive,

    Nothing,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            side_bar_active: false,
            dark_mode_active: true,
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
                if self.dark_mode_active == true{
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
                            if self.dark_mode_active == true{
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
                    <h1 class="user-name">{"Thaiminh2022"}</h1>
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

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::FlipSideBarActive => {
                // flip the side bar
                self.side_bar_active = !self.side_bar_active;
            }
            Msg::FlipDarkModeActive => {
                // flip the dark mode
                self.dark_mode_active = !self.dark_mode_active;
            }
            Msg::Nothing => (),
        }

        true
    }
}

fn main() {
    // The main file is for creating a nav bar
    yew::start_app::<App>();
}
