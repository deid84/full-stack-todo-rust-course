use crate::components::pages::count::Count;
use crate::components::pages::display::DisplayPage;
use crate::components::pages::hello::Hello;
use crate::components::pages::home::Home;
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::*;

#[derive(Debug, Clone, PartialEq, Routable)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/count")]
    Count,
    #[at("/display")]
    Display,
}

pub fn switch(route: &Route) -> Html {
    match route {
        Route::Home => html! { <Home /> },
        Route::Count => html! {<Count />},
        Route::Display => html! { <DisplayPage /> },
    }
}