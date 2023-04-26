use crate::components::atoms::custom_button::CustomButton;
use crate::components::atoms::main_title::{Color, MainTitle};
use crate::components::atoms::text_input::TextInput;
use gloo::console::log;
use wasm_bindgen::JsCast;
use web_sys::{Event, HtmlInputElement};
use yew::prelude::*;

#[function_component(CustomForm)]
pub fn custom_form() -> Html {
    let user_name_state = use_state(|| "No name set".to_string());
    let click_count_state = use_state(|| 0_u32);
    
    let cloned_user_name_state = user_name_state.clone();
    let username_changed = Callback::from(move |username: String| {
        log!("Username changed in custom form" , &username);
        cloned_user_name_state.set(username.to_string());
    });

    let colned_click_count_state = click_count_state.clone();
    let handel_button_click = Callback::from(move |_| {
        colned_click_count_state.set(*colned_click_count_state + 1);
    });

    html! {
        <div>
            <TextInput name={"Username"} handel_onchange={username_changed} />
            <CustomButton label="Click Here!!" onclick = {handel_button_click}/>
            <p>{"User Name : "}{&*user_name_state}</p>
            <p>{"Button clicked for "}{&*click_count_state}{ " of times"}</p>
        </div>
    }
}
