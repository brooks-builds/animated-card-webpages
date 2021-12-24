use super::rectangle::Rectangle;
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
                <Rectangle color={"rgb(0, 0, 40)"} z_index={100} height={"50vh"} />
                <Rectangle color={"rgb(0, 40, 0)"} z_index={10} height={"100vh"}/>
                <section class={"title"}>
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
        section.title {
            display: flex;
            height: 100vh;
            width: 100vw;
            justify-content: center;
            flex-direction: column;
            align-items: center;
            justify-content: space-between;
            position: absolute;
            z-index: 255;
        }

        .title span {
            color: green;
        }

        .title h1 {
            margin-top: 1rem;
            font-size: 5rem;
        }

        .title p {
            font-size: 3rem;
        }
    "#
    .into()
}
