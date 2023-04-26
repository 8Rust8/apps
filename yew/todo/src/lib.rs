#![allow(unused)]
pub mod components;
use components::atoms::main_title::{Color, MainTitle};

use gloo::console::log;
use serde::{Deserialize, Serialize};
use stylist::yew::styled_component;
use stylist::{style, Style};
use yew::prelude::*;
use yew::{classes, html};

#[styled_component(App)]
pub fn app() -> Html {
    // Once called back , it will log the message in console. The value travell back to here
    let main_title_callback = Callback::from(|message: String| log!(message));
    html! {
        <>
           <div>
            <MainTitle title={"Elephant"} color={Color::Error} on_load={main_title_callback} />
          </div>
        </>
    }
}
