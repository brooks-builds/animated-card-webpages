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
    pub size: f64,
    #[prop_or_default]
    pub class: Classes,
}

#[function_component(Icon)]
pub fn icon(props: &Props) -> Html {
    let style = Style::new(style_string(props)).unwrap();
    let style_name = style.get_class_name().to_owned();
    let style_string = style.get_style_str().to_owned();

    html! {
        <>
            <style>{style_string}</style>
            <div id="icon" class={&style_name}>
                {
                    match props.icon_type {
                        IconType::Tree => html! {<svg class="tree" xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24px" viewBox="0 0 24 24" width="24px" fill="#FFFFFF"><g><rect fill="none" height="24" width="24"/></g><g><g><g><path d="M17,12h2L12,2L5.05,12H7l-3.9,6h6.92v4h3.95v-4H21L17,12z M6.79,16l3.9-6H8.88l3.13-4.5l3.15,4.5h-1.9l4,6H6.79z"/></g></g></g></svg>},
                        IconType::Star => html! {<svg class="star" xmlns="http://www.w3.org/2000/svg" height="24px" viewBox="0 0 24 24" width="24px" fill="#FFFFFF"><path d="M0 0h24v24H0V0z" fill="none"/><path d="M12 17.27L18.18 21l-1.64-7.03L22 9.24l-7.19-.61L12 2 9.19 8.63 2 9.24l5.46 4.73L5.82 21 12 17.27z"/></svg>},
                        IconType::Snowflake => html! {<svg class="icon-snowflake" xmlns="http://www.w3.org/2000/svg" height="24px" viewBox="0 0 24 24" width="24px" fill="#FFFFFF"><path d="M0 0h24v24H0V0z" fill="none"/><path d="M22 11h-4.17l3.24-3.24-1.41-1.42L15 11h-2V9l4.66-4.66-1.42-1.41L13 6.17V2h-2v4.17L7.76 2.93 6.34 4.34 11 9v2H9L4.34 6.34 2.93 7.76 6.17 11H2v2h4.17l-3.24 3.24 1.41 1.42L9 13h2v2l-4.66 4.66 1.42 1.41L11 17.83V22h2v-4.17l3.24 3.24 1.42-1.41L13 15v-2h2l4.66 4.66 1.41-1.42L17.83 13H22v-2z"/></svg>},
                    }
                }
            </div>
        </>
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
            width: {size}px;
            height: {size}px;
            top: {top};
            left: {left};
        }}

        svg.star {{
            z-index: 190;
            fill: wheat;
            width: {size}px;
            height: {size}px;
            top: {top};
            left: {left};
        }}

        div {{
            z-index: 210;
            fill: white;
            width: {size}px;
            height: {size}px;
            top: {top};
            left: {left};
        }}
    ",
        top = &props.top,
        left = &props.left,
        size = &props.size,
    )
}
