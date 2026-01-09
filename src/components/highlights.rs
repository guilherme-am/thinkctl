use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
struct TitlePart {
    text: String,
    strike: bool,
}

fn parse_title_parts(input: &str) -> Vec<TitlePart> {
    let mut parts = Vec::new();
    let mut rest = input;

    while let Some(start) = rest.find("~~") {
        let before = &rest[..start];
        if !before.is_empty() {
            parts.push(TitlePart {
                text: before.to_string(),
                strike: false,
            });
        }

        let after_start = &rest[start + 2..];
        let Some(end) = after_start.find("~~") else {
            parts.push(TitlePart {
                text: "~~".to_string(),
                strike: false,
            });
            parts.push(TitlePart {
                text: after_start.to_string(),
                strike: false,
            });
            return parts;
        };

        let struck = &after_start[..end];
        parts.push(TitlePart {
            text: struck.to_string(),
            strike: true,
        });

        rest = &after_start[end + 2..];
    }

    if !rest.is_empty() {
        parts.push(TitlePart {
            text: rest.to_string(),
            strike: false,
        });
    }

    parts
}

#[component]
pub fn HighlightCard(title: &'static str, body: &'static str, icon_key: &'static str) -> Element {
    let icon_src = format!("https://skillicons.dev/icons?i={icon_key}&theme=dark");
    let title_parts = parse_title_parts(title);

    rsx! {
        div { class: "bg-slate-950 border border-slate-800 rounded-lg p-5 hover:border-green-500/50 transition-colors",
            div { class: "flex items-start gap-3",
                img { class: "stack-icon", src: "{icon_src}", alt: "{title}" }
                div {
                    div { class: "text-slate-200 font-mono text-sm",
                        for part in title_parts.iter() {
                            if part.strike {
                                span { class: "line-through text-slate-500", "{part.text}" }
                            } else {
                                span { "{part.text}" }
                            }
                        }
                    }
                    div { class: "text-slate-400 text-sm mt-2 leading-relaxed", "{body}" }
                }
            }
        }
    }
}

#[component]
pub fn HighlightsGrid(items: Vec<(&'static str, &'static str, &'static str)>) -> Element {
    rsx! {
        div { class: "grid md:grid-cols-2 gap-4",
            for (title, body, icon_key) in items {
                HighlightCard { title, body, icon_key }
            }
        }
    }
}
