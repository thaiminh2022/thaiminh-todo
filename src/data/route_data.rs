use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/login")]
    Login,
    #[at("/counter")]
    Counter,
    #[at("/todo")]
    Todo,
    #[not_found]
    #[at("/404")]
    NotFound,
}
