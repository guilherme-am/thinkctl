use dioxus::prelude::*;

#[component]
pub fn StackBadge(
    icon_key: &'static str,
    icon_url: Option<&'static str>,
    label: &'static str,
    color: &'static str,
) -> Element {
    let color_class = match color {
        "blue" => "text-blue-400",
        "purple" => "text-purple-400",
        "orange" => "text-orange-400",
        "red" => "text-red-500",
        _ => "text-green-400",
    };
    let icon_src = icon_url
        .map(|u| u.to_string())
        .unwrap_or_else(|| format!("https://skillicons.dev/icons?i={icon_key}&theme=dark"));

    rsx! {
        div {
            class: "flex items-center gap-2 bg-slate-900 border border-slate-800 p-2 rounded hover:border-green-500/50 transition-colors",
            img { class: "stack-icon", src: "{icon_src}", alt: "{label}" }
            span { class: format!("text-xs font-mono {}", color_class), "{label}" }
        }
    }
}

#[component]
pub fn SkillsGrid(
    items: Vec<(
        &'static str,
        &'static str,
        &'static str,
        Option<&'static str>,
    )>,
) -> Element {
    rsx! {
        div { class: "grid grid-cols-2 md:grid-cols-4 gap-4",
            for (key, label, color, icon_url) in items {
                StackBadge { icon_key: key, icon_url, label, color }
            }
        }
    }
}

#[component]
pub fn CertItem(label: &'static str, icon_url: Option<&'static str>) -> Element {
    rsx! {
        div { class: "bg-slate-950 border border-slate-800 rounded p-4 flex items-center gap-2",
            if let Some(src) = icon_url {
                img { class: "cert-icon", src: "{src}", alt: "" }
            } else {
                span { class: "text-slate-500 font-mono text-xs", "◆" }
            }
            div { class: "text-slate-300 text-sm", "{label}" }
        }
    }
}
