use dioxus::document::Link;
use dioxus::prelude::*;

mod components;
mod content;
mod content_index;
mod pages;
mod ui;

use pages::{Home, ThoughtsGrep, ThoughtsIndex, ThoughtsPath};

fn main() {
    launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        Link { rel: "stylesheet", href: "/style.css" }
        script { src: "/thoughts.js" }
        Router::<Route> {}
    }
}

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/")]
    Home {},

    #[route("/etc/thoughts")]
    ThoughtsIndex {},

    #[route("/etc/thoughts/grep")]
    ThoughtsGrep {},

    #[route("/etc/thoughts/:..segments")]
    ThoughtsPath { segments: Vec<String> },
}
