use crate::content_index::PostKind;
use dioxus::prelude::*;

#[component]
pub fn ThemeCard(
    title: String,
    description: String,
    items: String,
    color: &'static str,
    topic_slug: String,
    on_select: EventHandler<String>,
) -> Element {
    let border_color = match color {
        "blue" => "hover:border-blue-500",
        "purple" => "hover:border-purple-500",
        _ => "hover:border-green-500",
    };

    let text_color = match color {
        "blue" => "group-hover:text-blue-400",
        "purple" => "group-hover:text-purple-400",
        _ => "group-hover:text-green-400",
    };

    rsx! {
        div {
            class: format!("group cursor-pointer bg-slate-950 border border-slate-800 rounded-lg p-6 {} transition-all relative overflow-hidden", border_color),
            onclick: move |_| on_select.call(topic_slug.clone()),
            div {
                class: "absolute top-0 right-0 p-4 opacity-10 group-hover:opacity-20 transition-opacity",
                span { class: "text-6xl", "▣" }
            }
            h3 {
                class: format!("text-xl font-bold text-white mb-2 {} transition-colors", text_color),
                "{title}"
            }
            p {
                class: "text-slate-400 text-sm mb-4",
                "{description}"
            }
            div {
                class: "text-xs font-mono text-slate-500",
                "drwxr-xr-x {items}"
            }
        }
    }
}

pub fn kind_icon(kind: PostKind) -> &'static str {
    match kind {
        PostKind::Service => "●",
        PostKind::Conf => "◆",
        PostKind::Log => "▲",
        PostKind::Md => "■",
    }
}

pub fn kind_color(kind: PostKind) -> &'static str {
    match kind {
        PostKind::Service => "text-blue-400",
        PostKind::Conf => "text-purple-400",
        PostKind::Log => "text-orange-400",
        PostKind::Md => "text-slate-400",
    }
}
