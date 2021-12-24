use serde::Deserialize;
use stylist::Style;
use yew::prelude::*;
use yew_router::{history::Location, prelude::RouterScopeExt};

#[derive(Debug, Deserialize)]
struct QueryParams {
    #[serde(default = "default_name")]
    name: String,
    #[serde(default = "default_message")]
    message: String,
}

fn default_name() -> String {
    "Recipient Name".to_owned()
}

fn default_message() -> String {
    "To customize, set the query params 'name' and 'message'. For example /?name=John Doe&message=Merry Christmas!".to_owned()
}

pub struct Card {
    query_params: QueryParams,
    style: Style,
}

impl Component for Card {
    type Message = ();

    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let query_params: QueryParams = ctx.link().location().unwrap().query().unwrap();
        let style = Style::new(style()).unwrap();
        Self {
            query_params,
            style,
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let style_str = self.style.get_class_name().to_owned();
        html! {
            <div class={ &style_str }>
                <style>
                    { self.style.get_style_str() }
                </style>
                <section>
                    <h1>{ "Merry Christmas " }<span>{ &self.query_params.name }</span></h1>
                    <p>{ &self.query_params.message }</p>
                    <div></div> // hack for spacing with flex
                    <div></div>
                </section>
            </div>
        }
    }
}

fn style() -> String {
    r#"
        section {
            display: flex;
            height: 100vh;
            width: 100vw;
            justify-content: center;
            flex-direction: column;
            align-items: center;
            justify-content: space-between;
        }

        span {
            color: green;
        }

        h1 {
            margin-top: 1rem;
            font-size: 5rem;
        }

        p {
            font-size: 3rem;
        }
    "#
    .into()
}
