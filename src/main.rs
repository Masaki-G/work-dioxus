use dioxus::prelude::*;
extern crate log;



fn main() {
    env_logger::init();
    dioxus::desktop::launch(App);
}

fn App(cx: Scope) -> Element {
    let enabled = true;
    let mycolor = "red";
    let name = if enabled { "Jack" } else { "Bob" };


    cx.render(rsx! (
        button {
            width:"100px",
            height:"100px",
            background_color: "blue",
            onclick: move |_| log::debug!("ああああああああああああ"),




        }
    ))
}


