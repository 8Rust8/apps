use stylist::style;
use stylist::yew::styled_component;
use yew::prelude::*;

#[derive(PartialEq)]
pub enum Color {
    Normal,
    Ok,
    Error,
}

impl Color {
    pub fn color_to_string(&self) -> String {
        match self {
            Color::Normal => "normal".to_owned(),
            Color::Ok => "ok".to_owned(),
            Color::Error => "error".to_owned(),
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub title: String,
    pub color: Color,
    pub on_load: Callback<String>,
}

#[function_component(MainTitle)]
pub fn main_title(props: &Props) -> Html {
       let stylesheet = style!(
        r#"
        .normal { color: white}
        .ok { color: green }
        .error { color: red }
        "#
    )
    .unwrap();
    
    // calling back
    &props.on_load.emit("Calling back from main_title component".to_string());
    html! {
        <div class={stylesheet}>
            <h1 class={&props.color.color_to_string()}>{ &props.title }</h1>
        </div>
    }
}
