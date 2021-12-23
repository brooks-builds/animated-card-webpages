use serde::Deserialize;
use stylist::Style;
use yew::prelude::*;
use yew_router::{history::Location, prelude::RouterScopeExt};

#[derive(Debug, Deserialize)]
struct QueryParams {
    name: String,
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
            <section class={ &style_str }>
                <style>
                    { self.style.get_style_str() }
                </style>
                <h1>{ "Merry Christmas " }<span>{ &self.query_params.name }</span></h1>
            </section>
        }
    }
}

fn style() -> String {
    r#"
        span {
            color: green;
        }

        h1 {
            text-align: center;
        }
    "#
    .into()
}
