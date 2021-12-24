use stylist::Style;
use yew::prelude::*;

#[derive(Debug, PartialEq)]
pub enum IconType {
    Tree,
    Star,
    Snowflake,
}

#[derive(Debug, PartialEq, Properties)]
pub struct Props {
    pub icon_type: IconType,
}

#[function_component(Icon)]
pub fn icon(props: &Props) -> Html {
    let style = Style::new(style_string(props)).unwrap();
    let style_name = style.get_class_name().to_owned();
    let style_string = style.get_style_str().to_owned();
    let icon_type = "tree";

    html! {
        <div id="icon" class={&style_name}>
            <style>{style_string}</style>
            {
                match props.icon_type {
                    IconType::Tree => html! {<svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24px" viewBox="0 0 24 24" width="24px" fill="#FFFFFF"><g><rect fill="none" height="24" width="24"/></g><g><g><g><path d="M17,12h2L12,2L5.05,12H7l-3.9,6h6.92v4h3.95v-4H21L17,12z M6.79,16l3.9-6H8.88l3.13-4.5l3.15,4.5h-1.9l4,6H6.79z"/></g></g></g></svg>},
                    IconType::Star => html! {<svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24px" viewBox="0 0 24 24" width="24px" fill="#FFFFFF"><g><rect fill="none" height="24" width="24"/></g><g><g><g><path d="M17,12h2L12,2L5.05,12H7l-3.9,6h6.92v4h3.95v-4H21L17,12z M6.79,16l3.9-6H8.88l3.13-4.5l3.15,4.5h-1.9l4,6H6.79z"/></g></g></g></svg>},
                    IconType::Snowflake => html! {<svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24px" viewBox="0 0 24 24" width="24px" fill="#FFFFFF"><g><rect fill="none" height="24" width="24"/></g><g><g><g><path d="M17,12h2L12,2L5.05,12H7l-3.9,6h6.92v4h3.95v-4H21L17,12z M6.79,16l3.9-6H8.88l3.13-4.5l3.15,4.5h-1.9l4,6H6.79z"/></g></g></g></svg>},
                }
            }
        </div>
    }
}

fn style_string(_props: &Props) -> String {
    format!(
        "
        svg {{
            position: absolute;
            z-index: 200;
            fill: green;
            width: 200px;
            height: 200px;
            bottom: 50px;
            left: 200px;
        }}
    "
    )
}
