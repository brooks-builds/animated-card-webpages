use stylist::Style;
use yew::prelude::*;

#[derive(Debug, Properties, PartialEq)]
pub struct Props {
    pub color: String,
}

#[function_component(Rectangle)]
pub fn rectangle(props: &Props) -> Html {
    let style = Style::new(style_string(&props.color)).unwrap();
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

fn style_string(color: &str) -> String {
    format!(
        "
        section {{
            position: absolute;
            width: 100vw;
            height: 100vh;
            background-color: {};
        }}
    ",
        color
    )
}
