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
    pub top: String,
    pub left: String,
}

#[function_component(Icon)]
pub fn icon(props: &Props) -> Html {
    let style = Style::new(style_string(props)).unwrap();
    let style_name = style.get_class_name().to_owned();
    let style_string = style.get_style_str().to_owned();

    html! {
        <div id="icon" class={&style_name}>
            <style>{style_string}</style>
            {
                match props.icon_type {
                    IconType::Tree => html! {<svg class="tree" xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24px" viewBox="0 0 24 24" width="24px" fill="#FFFFFF"><g><rect fill="none" height="24" width="24"/></g><g><g><g><path d="M17,12h2L12,2L5.05,12H7l-3.9,6h6.92v4h3.95v-4H21L17,12z M6.79,16l3.9-6H8.88l3.13-4.5l3.15,4.5h-1.9l4,6H6.79z"/></g></g></g></svg>},
                    IconType::Star => html! {<svg class="star" xmlns="http://www.w3.org/2000/svg" height="24px" viewBox="0 0 24 24" width="24px" fill="#FFFFFF"><path d="M0 0h24v24H0V0z" fill="none"/><path d="M12 17.27L18.18 21l-1.64-7.03L22 9.24l-7.19-.61L12 2 9.19 8.63 2 9.24l5.46 4.73L5.82 21 12 17.27z"/></svg>},
                    IconType::Snowflake => html! {<svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24px" viewBox="0 0 24 24" width="24px" fill="#FFFFFF"><g><rect fill="none" height="24" width="24"/></g><g><g><g><path d="M17,12h2L12,2L5.05,12H7l-3.9,6h6.92v4h3.95v-4H21L17,12z M6.79,16l3.9-6H8.88l3.13-4.5l3.15,4.5h-1.9l4,6H6.79z"/></g></g></g></svg>},
                }
            }
        </div>
    }
}

fn style_string(props: &Props) -> String {
    format!(
        "
        svg {{
            position: absolute;
        }}
        svg.tree {{
            z-index: 200;
            fill: green;
            width: 200px;
            height: 200px;
            top: {top};
            left: {left};
        }}

        svg.star {{
            z-index: 190;
            fill: white;
            width: 25px;
            height: 25px;
            top: {top};
            left: {left};
        }}
    ",
        top = &props.top,
        left = &props.left,
    )
}
