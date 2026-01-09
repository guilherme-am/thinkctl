use crate::components::footer::Footer;
use crate::components::navbar::Navbar;
use crate::components::panel::Panel;
use crate::components::terminal_ls::TerminalLsRow;
use crate::content_index;
use crate::ui::terminal_ls;
use crate::ui::theme::Theme;
use dioxus::prelude::*;

#[component]
pub fn ThoughtsIndex() -> Element {
    let mut theme_state = use_signal(|| Theme::Midnight);
    let theme_value = *theme_state.read();

    rsx! {
        div { class: theme_value.class(),
            div { class: "scanline" }

            Navbar {
                theme_name: theme_value.name().to_string(),
                on_toggle_theme: move |_| {
                    let next = {
                        let current = *theme_state.read();
                        current.next()
                    };
                    theme_state.set(next);
                },
            }

            div { class: "bg-slate-950 relative z-10",
                section { class: "py-20 max-w-6xl mx-auto px-6",
                    Panel { title: "/etc/thoughts",
                        div { class: "text-slate-300 font-mono text-sm",
                            "Select a domain:"
                        }
                    }

                    div { class: "mt-10 grid md:grid-cols-3 gap-6",
                        for topic in content_index::topics().iter() {
                            a {
                                href: "/etc/thoughts/{topic.slug}",
                                style: "text-decoration: none;",
                                div {
                                    class: format!(
                                        "group cursor-pointer bg-slate-950 border border-slate-800 rounded-lg p-6 {} transition-all relative overflow-hidden",
                                        match topic.color {
                                            "blue" => "hover:border-blue-500",
                                            "purple" => "hover:border-purple-500",
                                            _ => "hover:border-green-500",
                                        }
                                    ),
                                    div {
                                        class: "absolute top-0 right-0 p-4 opacity-10 group-hover:opacity-20 transition-opacity",
                                        span { class: "text-6xl", "▣" }
                                    }
                                    h3 {
                                        class: format!(
                                            "text-xl font-bold text-white mb-2 {} transition-colors",
                                            match topic.color {
                                                "blue" => "group-hover:text-blue-400",
                                                "purple" => "group-hover:text-purple-400",
                                                _ => "group-hover:text-green-400",
                                            }
                                        ),
                                        "{topic.title}"
                                    }
                                    p { class: "text-slate-400 text-sm mb-4", "{topic.description}" }
                                    div { class: "text-xs font-mono text-slate-500",
                                        {format!("drwxr-xr-x {} items", topic.count)}
                                    }
                                }
                            }
                        }
                    }

                    div { class: "thoughts-scrollhint-wrap",
                        a { href: "#thoughts-ls", class: "thoughts-scrollhint font-mono",
                            span { class: "text-slate-500", "↓ " }
                            span { class: "text-slate-300", "scroll to ls -la output" }
                        }
                    }

                    div { id: "thoughts-ls", class: "mt-10 bg-slate-950 border border-slate-800 rounded-lg overflow-hidden",
                        div { class: "bg-slate-900 border-b border-slate-800 p-4 font-mono text-sm",
                            span { class: "text-slate-500", "$ " }
                            span { class: "text-green-400", "ls -la " }
                            span { class: "text-slate-300", "/etc/thoughts" }
                            span { class: "terminal-caret" }
                        }
                        div { class: "p-4 font-mono text-sm",
                            for post in content_index::root_posts().iter() {
                                TerminalLsRow { row: terminal_ls::row_for_post(post) }
                            }
                        }
                    }
                }

                Footer {}
            }
        }
    }
}
