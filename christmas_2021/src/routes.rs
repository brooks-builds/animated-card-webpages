use super::components::card::Card;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
}

pub fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => html! { <Card /> },
    }
}
