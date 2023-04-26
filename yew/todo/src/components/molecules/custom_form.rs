use std::clone;
use std::ops::Deref;

use crate::components::atoms::custom_button::CustomButton;
use crate::components::atoms::main_title::{Color, MainTitle};
use crate::components::atoms::text_input::TextInput;
use gloo::console::log;
use wasm_bindgen::JsCast;
use web_sys::{Event, HtmlInputElement};
use yew::prelude::*;

#[derive(Default, Clone)]
struct State {
    username: String,
    language: String,
    count: u32,
}

#[function_component(CustomForm)]
pub fn custom_form() -> Html {
    let state = use_state(|| State::default());

    let cloned_state = state.clone();
    let username_changed = Callback::from(move |username: String| {
        cloned_state.set(State {
            username,
            ..cloned_state.deref().clone()
        });
    });

    let cloned_state = state.clone();
    let language_changed = Callback::from(move |language: String| {
        cloned_state.set(State {
            language,
            ..cloned_state.deref().clone()
        });
    });

    let cloned_state = state.clone();
    let handel_button_click = Callback::from(move |_| {
        let count = cloned_state.count + 1;
        cloned_state.set(State {
            count,
            ..cloned_state.deref().clone()
        })
    });

    html! {
        <div>
            <div style="display: flex">
                <p>{"Username "} <TextInput name={"Username"} handel_onchange={username_changed} /></p>
                <p>{"Language "} <TextInput name={"Language"} handel_onchange={language_changed} /></p>
            </div>
            <div>
                <p>{"Username : "}{&state.username}</p>
                <p>{"Language : "}{&state.language}</p>
                <p>{"Button clicked for "}{&state.count}{ " of times"}</p>
            </div>
            <CustomButton label="Click Here!!" onclick = {handel_button_click}/>
        </div>
    }
}
