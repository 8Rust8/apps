#![allow(unused)]
pub mod components;
use components::atoms::main_title::{Color, MainTitle};
//use components::atoms::text_input::TextInput;
use components::molecules::custom_form::CustomForm;
use gloo::console::log;
//use serde::{Deserialize, Serialize};
use stylist::yew::styled_component;
use stylist::{style, Style};
use yew::prelude::*;
use yew::{classes, html};

#[styled_component(App)]
pub fn app() -> Html {
    // Once called back , it will log the message in console. The value travell back to here
    //  let lib_main_title_cb = Callback::from(|message: String| log!(message, " for lib"));
    html! {
        <>
        <div>
          <CustomForm />
        </div>
        </>

    }
}
