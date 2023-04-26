use crate::components::atoms::custom_button::CustomButton;
use crate::components::atoms::main_title::{Color, MainTitle};
use crate::components::atoms::text_input::TextInput;
use gloo::console::externs::log;
use gloo::console::log;
use wasm_bindgen::JsCast;
use web_sys::{Event, HtmlInputElement};
use yew::prelude::*;

#[function_component(CustomForm)]
pub fn custom_form() -> Html {
    let user_name_state = use_state(|| "No name set".to_string());
    let custom_form_main_title_cb = Callback::from(|message| log!(message, "for custom form"));
    
    let cloned_user_name_state = user_name_state.clone();
    let username_changed = Callback::from(move |username: String| {
        log!("Username changed in custom form" , &username);
        cloned_user_name_state.set(username.to_string());
    });

    html! {
        <form>
            <MainTitle title={"Yewwww"} color={Color::Normal} on_load={custom_form_main_title_cb} />
            <TextInput name={"Another elephant"} handel_onchange={username_changed} />
            <CustomButton label="Click Here!!"/>
            <p>{"User Name : "}{&*user_name_state}</p>
        </form>
    }
}
