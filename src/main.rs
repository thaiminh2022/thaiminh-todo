#![allow(dead_code)]
use yew::prelude::*;
use yew_router::prelude::*;

mod bridge;
mod comps;
mod custom;
mod data;

use comps::Counter;
use comps::Model;
use comps::NotFound;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/counter")]
    Counter,
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[function_component(App)]
fn final_app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={Switch::render(switch)} />
        </BrowserRouter>
    }
}

fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => html! {<Model/> },
        Route::NotFound => html! {<NotFound/>},
        Route::Counter => html! {<Counter/>},
    }
}

fn main() {
    yew::start_app::<App>();
}
