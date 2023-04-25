#![allow(unused)]
use gloo::console::log;
use serde::{Deserialize, Serialize};
use stylist::style;
use stylist::yew::styled_component;
use yew::prelude::*;
use yew::{classes, html};

#[derive(Serialize, Deserialize)]
struct MyObject {
    username: String,
    favourite_language: String,
}

#[function_component(App)]
pub fn app() -> Html {
    let name = "me";
    let my_object = MyObject {
        username: name.to_owned(),
        favourite_language: "rust".to_string(),
    };
    log!(serde_json::to_string_pretty(&my_object).unwrap());

    let message: Option<&str> = None;
    let tasks: Vec<&str> = vec!["Task One", "Task Two"];
    html! {
        <>
            <div class={classes!("hello")}>{"Hello Hello"}</div>
            if let Some(msg) = message {
                <p>{"Some messages"}</p>
            } else {
                <p>{"No messages"}</p>
            }

            <ul>
                {list_to_html(tasks)}
            </ul>
            <AppStyle />
        </>
    }
}

#[styled_component(AppStyle)]
pub fn app_style() -> Html {
    let stylesheet = style!(
        r#"
            h2 {
                color: red;
            }
            p1 {
                color: orange;
            }
        "#
    )
    .unwrap();

    html! {
        <div class={stylesheet}>
            <h2>{"Hello Red"}</h2>
            <p1>{"Hello Orange"}</p1>
            <h3 class={css!("color: white; font-size: 55px;")}>{"Hello inline css"}</h3>
        </div>
    }
}

fn list_to_html(list: Vec<&str>) -> Vec<Html> {
    // map one itterator into another
    // no need to collect to Html type as the return type of function dose the work
    list.iter().map(|task| html! {<li>{task}</li>}).collect()
}