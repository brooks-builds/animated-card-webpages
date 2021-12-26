mod components;
mod routes;
mod utilities;

use routes::{switch, Route};
use yew::prelude::*;
use yew_router::prelude::*;

pub struct App;

impl Component for App {
    type Message = ();

    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        // get the link information here and store it
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        // pass in the router information via a property
        html! {
            <BrowserRouter>
                <main>
                    <Switch<Route> render={Switch::render(switch)} />
                </main>
            </BrowserRouter>
        }
    }
}
