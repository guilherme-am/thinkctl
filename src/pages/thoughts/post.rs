use crate::components::footer::Footer;
use crate::components::navbar::Navbar;
use crate::components::panel::Panel;
use crate::components::themes;
use crate::content_index;
use crate::ui::markdown;
use crate::ui::theme::Theme;
use dioxus::prelude::*;

use super::topic::ThoughtsTopic;

#[component]
pub fn ThoughtsPath(segments: Vec<String>) -> Element {
    let mut theme_state = use_signal(|| Theme::Midnight);
    let theme_value = *theme_state.read();

    let topics = content_index::topics();
    let is_topic = segments.len() == 1 && topics.iter().any(|t| t.slug == segments[0]);

    if is_topic {
        let topic = segments[0].clone();
        return rsx! { ThoughtsTopic { topic } };
    }

    let slug = if let Some(first) = segments.first() {
        let is_topic_prefix = topics.iter().any(|t| t.slug == *first);
        if is_topic_prefix && segments.len() >= 2 {
            let topic = first.as_str();
            let rest = segments[1..].join("/");
            format!("{topic}/{rest}")
        } else {
            segments.join("/")
        }
    } else {
        String::new()
    };

    let Some(post) = content_index::get_post(&slug) else {
        return rsx! {
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
                        Panel { title: "not found",
                            div { class: "text-slate-300 font-mono text-sm",
                                "No unit found for: /etc/thoughts/{slug}"
                            }
                        }
                    }
                    Footer {}
                }
            }
        };
    };

    let (frontmatter, body) = markdown::split_frontmatter(&post.markdown);
    let title = frontmatter
        .and_then(markdown::frontmatter_title)
        .unwrap_or_else(|| post.title.clone());
    let tags = frontmatter
        .map(|fm| markdown::frontmatter_list(fm, "tags"))
        .unwrap_or_default();
    let minutes = estimate_reading_time_minutes(body);
    let rendered = markdown::render_markdown(body);
    let html = rendered.html;
    let toc = rendered.toc;

    let (kind_icon, kind_color) = (themes::kind_icon(post.kind), themes::kind_color(post.kind));

    let breadcrumb = if let Some((topic, rest)) = slug.split_once('/') {
        rsx! {
            a { href: "/etc/thoughts/{topic}", class: "text-slate-500 hover:text-green-400 transition-colors", style: "text-decoration: none;", "{topic}" }
            span { class: "text-slate-600", " / " }
            span { class: "text-slate-300", "{rest}" }
        }
    } else {
        rsx! {
            a { href: "/etc/thoughts", class: "text-slate-500 hover:text-green-400 transition-colors", style: "text-decoration: none;", "/etc/thoughts" }
            span { class: "text-slate-600", " / " }
            span { class: "text-slate-300", "{slug}" }
        }
    };

    let (prev, next, index_href) = neighbors(&slug);

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
                    div { class: "thoughts-layout",
                        main { class: "thoughts-main",
                            div { class: "thoughts-shell",
                                div { class: "thoughts-terminal-bar font-mono text-sm",
                                    div { class: "thoughts-terminal-left",
                                        span { class: "text-slate-500", "$ " }
                                        span { class: "text-green-400", "cat " }
                                        span { class: "text-slate-300", "/etc/thoughts/{slug}" }
                                    }
                                    div { class: "thoughts-terminal-right",
                                        span { class: kind_color, "{kind_icon}" }
                                        span { class: "text-slate-500 ml-2", "{kind_label(post.kind)}" }
                                    }
                                }

                                article { class: "thoughts-article",
                                    div { class: "thoughts-breadcrumb font-mono text-xs",
                                        {breadcrumb}
                                    }

                                    h1 { class: "thoughts-title", "{title}" }

                                    div { class: "thoughts-meta",
                                        div { class: "thoughts-meta-left",
                                            if minutes > 0 {
                                                span { class: "thoughts-pill", "{minutes} min read" }
                                            }
                                            span { class: "thoughts-pill",
                                                span { class: kind_color, "{kind_icon}" }
                                                span { class: "ml-2", "{kind_label(post.kind)}" }
                                            }
                                            if post.kind == content_index::PostKind::Service {
                                                span { class: "thoughts-pill thoughts-pill-active",
                                                    span { class: "thoughts-dot" }
                                                    "active"
                                                }
                                            }
                                        }
                                        div { class: "thoughts-meta-right",
                                            for tag in tags.iter() {
                                                span { class: "thoughts-tag", "#{tag}" }
                                            }
                                        }
                                    }

                                    div { class: "thoughts-prose",
                                        dangerous_inner_html: html
                                    }

                                    footer { class: "thoughts-footer-nav",
                                        a { class: "thoughts-navlink", href: "{index_href}",
                                            span { class: "thoughts-navcmd font-mono", "$ thinkctl list" }
                                            span { class: "thoughts-navtitle", "Back to index" }
                                        }
                                        if let Some(p) = prev {
                                            a { class: "thoughts-navlink", href: "/etc/thoughts/{p.slug}",
                                                span { class: "thoughts-navcmd font-mono", "$ thinkctl prev" }
                                                span { class: "thoughts-navtitle", "{p.title}" }
                                            }
                                        }
                                        if let Some(n) = next {
                                            a { class: "thoughts-navlink thoughts-navlink-right", href: "/etc/thoughts/{n.slug}",
                                                span { class: "thoughts-navcmd font-mono", "$ thinkctl next" }
                                                span { class: "thoughts-navtitle", "{n.title}" }
                                            }
                                        }
                                    }
                                }
                            }
                        }

                        aside { class: "thoughts-sidebar",
                            if !toc.is_empty() {
                                div { class: "thoughts-sidebar-block",
                                    h3 { class: "thoughts-sidebar-title font-mono", "Contents" }
                                    nav { id: "toc", class: "thoughts-toc",
                                        for item in toc.iter() {
                                            a {
                                                class: if item.level == 3 { "toc-link toc-link-h3" } else { "toc-link" },
                                                href: "#{item.id}",
                                                "{item.title}"
                                            }
                                        }
                                    }
                                }
                            }

                            div { class: "thoughts-sidebar-block",
                                h3 { class: "thoughts-sidebar-title font-mono", "Related Units" }
                                div { class: "thoughts-related",
                                    for rel in related_units(&slug).iter() {
                                        a { href: "/etc/thoughts/{rel.slug}", class: "thoughts-related-link",
                                            div { class: "thoughts-related-top font-mono",
                                                span { class: format!("text-xs {}", themes::kind_color(rel.kind)), "{themes::kind_icon(rel.kind)}" }
                                                span { class: "text-slate-500", "{unit_label(rel)}" }
                                            }
                                            div { class: "thoughts-related-title", "{rel.title}" }
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

fn unit_label(post: &content_index::PostRef) -> String {
    let name = post.slug.split('/').next_back().unwrap_or(&post.slug);
    let ext = match post.kind {
        content_index::PostKind::Service => "service",
        content_index::PostKind::Conf => "conf",
        content_index::PostKind::Log => "log",
        content_index::PostKind::Md => "md",
    };
    format!("{name}.{ext}")
}

fn neighbors(
    slug: &str,
) -> (
    Option<content_index::PostRef>,
    Option<content_index::PostRef>,
    String,
) {
    if let Some((topic, _)) = slug.split_once('/') {
        let posts = content_index::posts_for(topic);
        let idx = posts.iter().position(|p| p.slug == slug);
        let prev = idx
            .and_then(|i| i.checked_sub(1))
            .and_then(|i| posts.get(i))
            .cloned();
        let next = idx.and_then(|i| posts.get(i + 1)).cloned();
        let index_href = format!("/etc/thoughts/{topic}");
        return (prev, next, index_href);
    }

    let posts = content_index::root_posts();
    let idx = posts.iter().position(|p| p.slug == slug);
    let prev = idx
        .and_then(|i| i.checked_sub(1))
        .and_then(|i| posts.get(i))
        .cloned();
    let next = idx.and_then(|i| posts.get(i + 1)).cloned();
    (prev, next, "/etc/thoughts".to_string())
}

fn related_units(slug: &str) -> Vec<content_index::PostRef> {
    let Some((topic, _)) = slug.split_once('/') else {
        return Vec::new();
    };

    content_index::posts_for(topic)
        .into_iter()
        .filter(|p| p.slug != slug)
        .take(5)
        .collect()
}

fn estimate_reading_time_minutes(markdown: &str) -> usize {
    let words = markdown.split_whitespace().count();
    let minutes = (words as f32 / 220.0).ceil() as usize;
    minutes.max(1)
}

fn kind_label(kind: content_index::PostKind) -> &'static str {
    match kind {
        content_index::PostKind::Service => "service",
        content_index::PostKind::Conf => "conf",
        content_index::PostKind::Log => "log",
        content_index::PostKind::Md => "md",
    }
}
