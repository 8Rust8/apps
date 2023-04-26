use gloo::console::log;
use std::rc::Rc;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub name: String,
    pub handel_onchange: Callback<String>,
}

#[function_component(TextInput)]
pub fn text_input(props: &Props) -> Html {
    // here handel_onchange is the callback function passesd to the component
    let cb_handel = props.handel_onchange.clone();

    let onchange = Callback::from(move |e: Event| {
        let value = e
            .target()
            .unwrap()
            .unchecked_into::<HtmlInputElement>()
            .value();
        cb_handel.emit(value);
    });

    html! {
        <>
            <input name= {props.name.to_owned()} onchange={onchange} />
        </>
    }
}
