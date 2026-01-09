use dioxus::prelude::*;

#[component]
pub fn SectionHeader(label: &'static str, title: &'static str) -> Element {
    rsx! {
        h2 { class: "text-3xl font-bold text-white mb-6 flex items-center gap-3",
            span { class: "text-green-500", "▶" }
            span { class: "font-mono", "{label}" }
            span { class: "sr-only", "{title}" }
        }
    }
}
