use crate::components::footer::Footer;
use crate::components::navbar::Navbar;
use crate::components::panel::Panel;
use crate::components::terminal_ls::TerminalLsRow;
use crate::content_index;
use crate::ui::terminal_ls;
use crate::ui::theme::Theme;
use dioxus::prelude::*;

#[component]
pub fn ThoughtsTopic(topic: String) -> Element {
    let mut theme_state = use_signal(|| Theme::Midnight);
    let theme_value = *theme_state.read();

    let posts = content_index::posts_for(&topic);
    let topic_label = content_index::titleize_public(&topic);

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
                section { class: "py-20 max-w-5xl mx-auto px-6",
                    Panel { title: "/etc/thoughts",
                        div { class: "text-slate-300 font-mono text-sm",
                            span { class: "text-slate-500", "cwd: " }
                            span { class: "text-green-400", "/etc/thoughts/{topic_label}" }
                        }
                    }

                    div { id: "thoughts-ls", class: "mt-10 bg-slate-950 border border-slate-800 rounded-lg overflow-hidden",
                        div { class: "bg-slate-900 border-b border-slate-800 p-4 font-mono text-sm",
                            span { class: "text-slate-500", "$ " }
                            span { class: "text-green-400", "ls -la " }
                            span { class: "text-slate-300", "/etc/thoughts/{topic_label}" }
                            span { class: "terminal-caret" }
                        }
                        div { class: "p-4 font-mono text-sm",
                            for post in posts.iter() {
                                TerminalLsRow { row: terminal_ls::row_for_post(post) }
                            }
                        }
                    }

                    div { class: "mt-10",
                        a {
                            href: "/etc/thoughts",
                            class: "text-sm font-mono text-green-400 cursor-pointer hover:underline",
                            style: "text-decoration: none;",
                            "cd /etc/thoughts"
                        }
                    }
                }

                Footer {}
            }
        }
    }
}
