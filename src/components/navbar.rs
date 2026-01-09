use dioxus::prelude::*;

#[cfg(target_arch = "wasm32")]
fn url_encode_component(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    for b in input.bytes() {
        match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                out.push(b as char);
            }
            b' ' => out.push_str("%20"),
            _ => out.push_str(&format!("%{:02X}", b)),
        }
    }
    out
}

#[cfg(target_arch = "wasm32")]
fn navigate_to_grep(query: &str) {
    use web_sys::window;

    let Some(win) = window() else {
        return;
    };

    let encoded = url_encode_component(query);
    let href = format!("/etc/thoughts/grep?q={encoded}");
    let _ = win.location().set_href(&href);
}

#[cfg(not(target_arch = "wasm32"))]
fn navigate_to_grep(_query: &str) {}

#[component]
pub fn Navbar(theme_name: String, on_toggle_theme: EventHandler<()>) -> Element {
    let mut mobile_open = use_signal(|| false);
    let mut grep_query = use_signal(String::new);

    rsx! {
        nav {
            class: "fixed top-0 w-full glass-panel border-b border-slate-800",
            style: "z-index: 50;",
            div {
                class: "max-w-6xl mx-auto px-6",
                div {
                    class: "flex items-center justify-between",
                    style: "height: 4rem;",
                    a {
                        class: "font-mono text-green-400 font-bold",
                        style: "font-size: 1.125rem; text-decoration: none;",
                        href: "/",
                        "> thinkctl_"
                    }
                    div {
                        class: "flex items-center gap-3 font-mono text-sm",

                        div { class: "hidden md:flex items-center gap-6",
                            a {
                                class: "text-slate-300 hover:text-green-400 transition-colors",
                                style: "text-decoration: none;",
                                href: "/",
                                "~/home"
                            }
                            a {
                                class: "text-slate-300 hover:text-green-400 transition-colors",
                                style: "text-decoration: none;",
                                href: "/etc/thoughts",
                                "/etc/thoughts"
                            }
                        }

                        form {
                            class: "hidden md:flex items-center",
                            action: "/etc/thoughts/grep",
                            method: "get",
                            div { class: "nav-search",
                                svg {
                                    width: "14",
                                    height: "14",
                                    view_box: "0 0 24 24",
                                    fill: "none",
                                    stroke: "currentColor",
                                    stroke_width: "2",
                                    stroke_linecap: "round",
                                    stroke_linejoin: "round",
                                    path { d: "M21 21l-4.35-4.35" }
                                    circle { cx: "11", cy: "11", r: "7" }
                                }
                                input {
                                    class: "nav-search-input",
                                    name: "q",
                                    value: "{grep_query.read()}",
                                    placeholder: "grep /etc/thoughts…",
                                    oninput: move |e| grep_query.set(e.value()),
                                    onkeydown: move |evt| {
                                        if evt.key() != Key::Enter {
                                            return;
                                        }
                                        evt.prevent_default();
                                        let q = grep_query.read().trim().to_string();
                                        if q.is_empty() {
                                            return;
                                        }
                                        navigate_to_grep(&q);
                                    }
                                }
                                button {
                                    class: "nav-button",
                                    r#type: "button",
                                    onclick: move |_| {
                                        let q = grep_query.read().trim().to_string();
                                        if q.is_empty() {
                                            return;
                                        }
                                        navigate_to_grep(&q);
                                    },
                                    svg {
                                        width: "14",
                                        height: "14",
                                        view_box: "0 0 24 24",
                                        fill: "none",
                                        stroke: "currentColor",
                                        stroke_width: "2",
                                        stroke_linecap: "round",
                                        stroke_linejoin: "round",
                                        path { d: "M21 21l-4.35-4.35" }
                                        circle { cx: "11", cy: "11", r: "7" }
                                    }
                                }
                            }
                        }

                        a {
                            class: "nav-button hidden md:flex items-center gap-2",
                            style: "text-decoration: none;",
                            href: "https://github.com/guilherme-am/thinkctl",
                            target: "_blank",
                            svg {
                                width: "14",
                                height: "14",
                                view_box: "0 0 16 16",
                                fill: "currentColor",
                                path {
                                    d: "M8 0C3.58 0 0 3.58 0 8c0 3.54 2.29 6.53 5.47 7.59.4.07.55-.17.55-.38 0-.19-.01-.82-.01-1.49-2.01.37-2.53-.49-2.69-.94-.09-.23-.48-.94-.82-1.13-.28-.15-.68-.52-.01-.53.63-.01 1.08.58 1.23.82.72 1.21 1.87.87 2.33.66.07-.52.28-.87.51-1.07-1.78-.2-3.64-.89-3.64-3.95 0-.87.31-1.59.82-2.15-.08-.2-.36-1.02.08-2.12 0 0 .67-.21 2.2.82.64-.18 1.32-.27 2-.27.68 0 1.36.09 2 .27 1.53-1.04 2.2-.82 2.2-.82.44 1.1.16 1.92.08 2.12.51.56.82 1.27.82 2.15 0 3.07-1.87 3.75-3.65 3.95.29.25.54.73.54 1.48 0 1.07-.01 1.93-.01 2.2 0 .21.15.46.55.38A8.012 8.012 0 0 0 16 8c0-4.42-3.58-8-8-8Z"
                                }
                            }
                            "guilherme-am/thinkctl"
                            span { class: "text-slate-500", "★" }
                        }

                        button {
                            class: "nav-button",
                            onclick: move |_| on_toggle_theme.call(()),
                            "theme: {theme_name}"
                        }

                        a {
                            class: "nav-button md:hidden",
                            style: "text-decoration: none;",
                            href: "/etc/thoughts/grep",
                            svg {
                                width: "14",
                                height: "14",
                                view_box: "0 0 24 24",
                                fill: "none",
                                stroke: "currentColor",
                                stroke_width: "2",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                path { d: "M21 21l-4.35-4.35" }
                                circle { cx: "11", cy: "11", r: "7" }
                            }
                        }

                        button {
                            class: "nav-button md:hidden",
                            onclick: move |_| {
                                let next = {
                                    let current = *mobile_open.read();
                                    !current
                                };
                                mobile_open.set(next);
                            },
                            if *mobile_open.read() { "menu: on" } else { "menu: off" }
                        }
                    }
                }
            }

            if *mobile_open.read() {
                div { class: "md:hidden border-t border-slate-800 bg-slate-950/95",
                    div { class: "max-w-6xl mx-auto px-6 py-4 flex flex-col gap-3 font-mono text-sm",
                        a {
                            class: "text-slate-300 hover:text-green-400 transition-colors",
                            style: "text-decoration: none;",
                            href: "/",
                            onclick: move |_| mobile_open.set(false),
                            "~/home"
                        }
                        a {
                            class: "text-slate-300 hover:text-green-400 transition-colors",
                            style: "text-decoration: none;",
                            href: "/etc/thoughts",
                            onclick: move |_| mobile_open.set(false),
                            "/etc/thoughts"
                        }
                        a {
                            class: "text-slate-300 hover:text-green-400 transition-colors",
                            style: "text-decoration: none;",
                            href: "/etc/thoughts/grep",
                            onclick: move |_| mobile_open.set(false),
                            "grep"
                        }
                    }
                }
            }
        }
    }
}
