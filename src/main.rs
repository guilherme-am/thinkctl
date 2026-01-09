use dioxus::document::Link;
use dioxus::prelude::*;

mod components;
mod content;
mod content_index;
mod pages;
mod ui;

use pages::{Home, ThoughtsGrep, ThoughtsIndex, ThoughtsPath};

fn main() {
    #[cfg(target_arch = "wasm32")]
    apply_spa_redirect();
    launch(App);
}

#[cfg(target_arch = "wasm32")]
fn apply_spa_redirect() {
    use wasm_bindgen::JsValue;
    use web_sys::window;

    fn decode_component(s: &str) -> String {
        let bytes = s.as_bytes();
        let mut out: Vec<u8> = Vec::with_capacity(bytes.len());
        let mut i = 0;
        while i < bytes.len() {
            match bytes[i] {
                b'+' => {
                    out.push(b' ');
                    i += 1;
                }
                b'%' if i + 2 < bytes.len() => {
                    let h1 = bytes[i + 1];
                    let h2 = bytes[i + 2];
                    let hex = |b: u8| -> Option<u8> {
                        match b {
                            b'0'..=b'9' => Some(b - b'0'),
                            b'a'..=b'f' => Some(b - b'a' + 10),
                            b'A'..=b'F' => Some(b - b'A' + 10),
                            _ => None,
                        }
                    };
                    if let (Some(a), Some(b)) = (hex(h1), hex(h2)) {
                        out.push((a << 4) | b);
                        i += 3;
                    } else {
                        out.push(bytes[i]);
                        i += 1;
                    }
                }
                _ => {
                    out.push(bytes[i]);
                    i += 1;
                }
            }
        }
        String::from_utf8_lossy(&out).to_string()
    }

    let Some(win) = window() else {
        return;
    };
    let search = win.location().search().ok().unwrap_or_default();
    let search = search.strip_prefix('?').unwrap_or(search.as_str());
    if search.is_empty() {
        return;
    }

    let mut route: Option<String> = None;
    for pair in search.split('&') {
        let (k, v) = pair.split_once('=').unwrap_or((pair, ""));
        if k == "__route__" {
            let decoded = decode_component(v);
            if !decoded.trim().is_empty() {
                route = Some(decoded);
                break;
            }
        }
    }

    let Some(route) = route else {
        return;
    };

    if let Ok(history) = win.history() {
        let _ = history.replace_state_with_url(&JsValue::NULL, "", Some(&route));
    }
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
