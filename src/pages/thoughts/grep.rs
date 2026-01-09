use crate::components::footer::Footer;
use crate::components::navbar::Navbar;
use crate::content_index;
use crate::ui::theme::Theme;
use dioxus::prelude::*;

use super::url::initial_query_from_url;

#[component]
pub fn ThoughtsGrep() -> Element {
    let mut theme_state = use_signal(|| Theme::Midnight);
    let theme_value = *theme_state.read();

    let mut query = use_signal(String::new);
    let results = content_index::search(query.read().as_str());

    use_effect(move || {
        if query.read().trim().is_empty()
            && let Some(q) = initial_query_from_url()
        {
            query.set(q);
        }
    });

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
                    div { class: "thoughts-shell",
                        div { class: "thoughts-terminal-bar font-mono text-sm",
                            div { class: "thoughts-terminal-left",
                                span { class: "text-slate-500", "$ " }
                                span { class: "text-green-400", "grep " }
                                span { class: "text-slate-300", "/etc/thoughts -R \"{query.read()}\"" }
                            }
                            div { class: "thoughts-terminal-right",
                                span { class: "text-slate-500", "search" }
                            }
                        }

                        div { class: "thoughts-article",
                            h1 { class: "thoughts-title", "grep" }
                            div { class: "thoughts-meta",
                                div { class: "thoughts-meta-left",
                                    span { class: "thoughts-pill", "Search across /etc/thoughts" }
                                }
                            }

                            div { class: "thoughts-search",
                                input {
                                    class: "thoughts-search-input font-mono",
                                    value: "{query.read()}",
                                    placeholder: "query (slug or title)…",
                                    oninput: move |e| query.set(e.value()),
                                }
                            }

                            if !query.read().trim().is_empty() {
                                div { class: "mt-10 bg-slate-950 border border-slate-800 rounded-lg overflow-hidden",
                                    div { class: "bg-slate-900 border-b border-slate-800 p-4 font-mono text-sm",
                                        span { class: "text-slate-500", "$ " }
                                        span { class: "text-green-400", "rg " }
                                        span { class: "text-slate-300", "\"{query.read()}\" /etc/thoughts" }
                                    }
                                    div { class: "p-4 font-mono text-sm",
                                        if results.is_empty() {
                                            div { class: "text-slate-500", "no matches" }
                                        }
                                        for post in results.iter() {
                                            a {
                                                href: "/etc/thoughts/{post.slug}",
                                                style: "text-decoration: none;",
                                                class: "flex items-center justify-between py-1 border-b border-slate-800 hover:text-green-400 transition-colors",
                                                div { class: "flex items-center gap-2",
                                                    span { class: format!("text-xs {}", crate::components::themes::kind_color(post.kind)), "{crate::components::themes::kind_icon(post.kind)}" }
                                                    span { class: "text-slate-300", "{post.title}" }
                                                }
                                                span { class: "text-slate-500", "{post.slug}" }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                Footer {}
            }
        }
    }
}
