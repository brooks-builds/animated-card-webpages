use crate::{components::icon::IconType, utilities::random_between};

use super::icon::Icon;
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

impl Card {
    fn create_forest(&self, tree_count: u8) -> Vec<Html> {
        let mut trees = vec![];

        for _ in 0..tree_count {
            let top = format!("{}%", random_between(39.0, 84.0));
            let left = format!("{}%", random_between(-1.0, 88.0));
            trees.push(html! {<Icon icon_type={IconType::Tree} top={top} left={left} />});
        }

        trees
    }

    fn create_stars(&self, star_count: u8) -> Vec<Html> {
        let mut stars = vec![];

        for _ in 0..star_count {
            let top = "10%";
            let left = "10%";
            stars.push(html! { <Icon icon_type={ IconType::Star } top={ top } left = { left } /> })
        }

        stars
    }
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
                { self.create_forest(25) }
                { self.create_stars(1) }
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
            color: white;
        }

        .title p {
            font-size: 3rem;
            color: white;
        }
    "#
    .into()
}
