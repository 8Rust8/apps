
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub label: String,
    pub onclick: Callback<()>
}

#[function_component(CustomButton)]
pub fn custom_button(props: &Props) -> Html {
    let cloned_click = props.onclick.clone();
    let onclick = Callback::from(move |_| {
        cloned_click.emit(());
    });
    html! {
        <>
            <button name="custombutton" onclick={onclick}>{&props.label.clone()}</button>
        </>
    }
}