use stylist::Style;
use yew::prelude::*;

#[derive(Debug, Properties, PartialEq)]
pub struct Props {
    pub color: String,
    pub z_index: u8,
    pub height: String,
}

#[function_component(Rectangle)]
pub fn rectangle(props: &Props) -> Html {
    let style = Style::new(style_string(&props.color, props.z_index, &props.height)).unwrap();
    let style_class_name = style.get_class_name().to_owned();
    let styles = style.get_style_str().to_owned();

    html! {
        <div class={ &style_class_name }>
            <style>{ &styles }</style>
            <section>
            </section>
        </div>
    }
}

fn style_string(color: &str, z_index: u8, height: &str) -> String {
    format!(
        "
        section {{
            position: absolute;
            width: 100vw;
            height: {};
            background-color: {};
            z-index: {};
        }}
    ",
        height, color, z_index,
    )
}
