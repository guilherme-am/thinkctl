use crate::components::footer::Footer;
use crate::components::highlights::HighlightsGrid;
use crate::components::navbar::Navbar;
use crate::components::panel::Panel;
use crate::components::section_header::SectionHeader;
use crate::components::skills::{CertItem, SkillsGrid};
use crate::components::terminal::Terminal;
use crate::components::terminal_ls::TerminalLsRow;
use crate::components::themes::ThemeCard;
use crate::content::profile;
use crate::content_index;
use crate::ui::terminal_ls;
use crate::ui::theme::Theme;
use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    let mut theme = use_signal(|| Theme::Midnight);
    let mut selected_topic = use_signal(|| Option::<String>::None);
    let topics = content_index::topics();
    let theme_value = *theme.read();
    let theme_name = theme_value.name().to_string();
    let theme_class = theme_value.class();
    let selected_topic_label = selected_topic
        .read()
        .as_deref()
        .map(content_index::titleize_public)
        .unwrap_or_default();
    let selected_posts = selected_topic
        .read()
        .as_deref()
        .map(content_index::posts_for)
        .unwrap_or_default();
    let show_selected_posts = selected_topic.read().is_some();

    rsx! {
        div { class: theme_class,

        div { class: "scanline" }

            Navbar {
                theme_name,
                on_toggle_theme: move |_| {
                    let next = {
                        let current = *theme.read();
                        current.next()
                    };
                    theme.set(next);
                },
            }

        section {
            class: "min-h-screen flex flex-col items-center justify-center px-4 relative bg-grid-pattern",
                style: "padding-top: 5rem;",

                Panel { title: "user@thinkctl:~",
                Terminal {}
            }
        }

        div {
            class: "bg-slate-950 relative z-10",
                id: "main-content",

            section {
                class: "py-20 max-w-5xl mx-auto px-6",
                div {
                    class: "flex flex-col md:flex-row gap-12 items-start",

                    div {
                        class: "md:w-1/3 relative",
                        div {
                            class: "aspect-square rounded-2xl overflow-hidden border-2 border-slate-700 relative group",
                                img {
                                    class: "w-full h-full object-cover grayscale group-hover:grayscale-0 transition-all duration-500",
                                    src: "https://avatars.githubusercontent.com/u/73472392?v=4",
                                    alt: "usr: guilherme"
                            }
                            div {
                                class: "absolute inset-0 bg-gradient-to-t from-slate-900 to-transparent opacity-60"
                            }
                            div {
                                class: "absolute bottom-4 left-4 font-mono text-xs text-green-400 bg-slate-900/80 px-2 py-1 rounded border border-green-900",
                                "usr: guilherme"
                            }
                        }
                    }

                    div {
                        class: "md:w-2/3",
                            id: "whoami",
                            SectionHeader { label: "whoami.conf", title: "whoami" }
                        div {
                            class: "text-slate-300 space-y-4",
                                p { "{profile::WHOAMI_PARAGRAPHS[0]}" }
                                p { "{profile::WHOAMI_PARAGRAPHS[1]}" }
                                p { "{profile::WHOAMI_PARAGRAPHS[2]}" }
                        }

                        div {
                            class: "mt-10",
                            h3 {
                                class: "text-sm font-mono text-slate-500 uppercase tracking-wider mb-4",
                                    "What drives me"
                                }
                                HighlightsGrid {
                                    items: profile::HIGHLIGHTS
                                        .iter()
                                        .map(|(t, b, i)| (*t, *b, *i))
                                        .collect(),
                                }
                            }

                            div {
                                class: "mt-10",
                                id: "stack",
                                h3 {
                                    class: "text-sm font-mono text-slate-500 uppercase tracking-wider mb-4",
                                    "Expertise & Stack"
                                }
                                div { class: "space-y-12",
                                    div {
                                        h4 { class: "text-xs font-mono text-slate-500 uppercase tracking-wider mb-4", "Cloud & Platform" }
                                        SkillsGrid {
                                            items: profile::SKILLS_PLATFORM
                                                .iter()
                                                .map(|(k, l, c, u)| (*k, *l, *c, *u))
                                                .collect(),
                                        }
                                    }

                                    div { class: "pt-2",
                                        h4 { class: "text-xs font-mono text-slate-500 uppercase tracking-wider mb-4", "Containers / Orchestration" }
                                        SkillsGrid {
                                            items: profile::SKILLS_CONTAINERS
                                                .iter()
                                                .map(|(k, l, c, u)| (*k, *l, *c, *u))
                                                .collect(),
                                        }
                                    }

                                    div { class: "pt-2",
                                        h4 { class: "text-xs font-mono text-slate-500 uppercase tracking-wider mb-4", "IaC & CI/CD" }
                                        SkillsGrid {
                                            items: profile::SKILLS_IAC_CICD
                                                .iter()
                                                .map(|(k, l, c, u)| (*k, *l, *c, *u))
                                                .collect(),
                                        }
                                    }

                                    div { class: "pt-2",
                                        h4 { class: "text-xs font-mono text-slate-500 uppercase tracking-wider mb-4", "Observability" }
                                        SkillsGrid {
                                            items: profile::SKILLS_OBSERVABILITY
                                                .iter()
                                                .map(|(k, l, c, u)| (*k, *l, *c, *u))
                                                .collect(),
                                        }
                                    }

                                    div { class: "pt-2",
                                        h4 { class: "text-xs font-mono text-slate-500 uppercase tracking-wider mb-4", "AI / MLOps tooling" }
                                        SkillsGrid {
                                            items: profile::SKILLS_AI_MLOPS
                                                .iter()
                                                .map(|(k, l, c, u)| (*k, *l, *c, *u))
                                                .collect(),
                                        }
                                    }

                                    div { class: "pt-2",
                                        h4 { class: "text-xs font-mono text-slate-500 uppercase tracking-wider mb-4", "Languages" }
                                        SkillsGrid {
                                            items: profile::SKILLS_LANGUAGES
                                                .iter()
                                                .map(|(k, l, c, u)| (*k, *l, *c, *u))
                                                .collect(),
                                        }
                                    }

                                    div { class: "pt-2",
                                        h4 { class: "text-xs font-mono text-slate-500 uppercase tracking-wider mb-4", "Databases" }
                                        SkillsGrid {
                                            items: profile::SKILLS_DATABASES
                                                .iter()
                                                .map(|(k, l, c, u)| (*k, *l, *c, *u))
                                                .collect(),
                                        }
                                    }
                                }
                            }

                            div {
                                class: "mt-10",
                                id: "certs",
                                h3 {
                                    class: "text-sm font-mono text-slate-500 uppercase tracking-wider mb-4",
                                    "Certifications & Training"
                                }
                                div { class: "grid md:grid-cols-2 gap-4 neural-panel",
                                    for (label, icon_url) in profile::CERTS {
                                        CertItem { label, icon_url }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            section {
                class: "py-20 bg-slate-900 border-t border-slate-800",
                id: "themes",
                div {
                    class: "max-w-6xl mx-auto px-6",
                    div {
                        class: "flex items-center justify-between mb-12",
                        h2 {
                            class: "text-2xl font-bold font-mono text-white",
                            "/etc/thoughts/themes"
                        }
                        span {
                            class: "text-sm font-mono text-green-400 cursor-pointer hover:underline",
                            a {
                                href: "/etc/thoughts",
                                style: "text-decoration: none;",
                                class: "text-green-400",
                                "view_all_units -a"
                            }
                        }
                    }
                    div {
                        class: "grid md:grid-cols-3 gap-6",
                        for topic in topics.iter() {
                        ThemeCard {
                                title: topic.title.clone(),
                                description: topic.description.clone(),
                                items: format!("{} items", topic.count),
                                color: topic.color,
                                topic_slug: topic.slug.clone(),
                                on_select: move |slug| {
                                    selected_topic.set(Some(slug));
                                },
                            }
                        }
                    }

                    if show_selected_posts {
                        div { class: "mt-10 bg-slate-950 border border-slate-800 rounded-lg overflow-hidden",
                            div { class: "bg-slate-900 border-b border-slate-800 p-4 font-mono text-sm",
                                span { class: "text-slate-500", "$ " }
                                span { class: "text-green-400", "ls -la " }
                                span { class: "text-slate-300", "/etc/thoughts/{selected_topic_label}" }
                                span { class: "terminal-caret" }
                            }
                            div { class: "p-4 font-mono text-sm",
                                for post in selected_posts.iter() {
                                    TerminalLsRow { row: terminal_ls::row_for_post(post) }
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
