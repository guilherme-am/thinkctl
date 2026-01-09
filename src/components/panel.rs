use dioxus::prelude::*;

#[component]
pub fn Panel(title: &'static str, children: Element) -> Element {
    rsx! {
        div { class: "w-full bg-slate-900 rounded-lg shadow-2xl border border-slate-700 overflow-hidden relative",
            PanelHeader { title }
            {children}
        }
    }
}

#[component]
pub fn PanelHeader(title: &'static str) -> Element {
    rsx! {
        div { class: "bg-slate-800 px-4 py-2 flex items-center justify-between border-b border-slate-700",
            div { class: "flex items-center gap-2",
                div { class: "w-3 h-3 rounded-full bg-red-500" }
                div { class: "w-3 h-3 rounded-full bg-yellow-500" }
                div { class: "w-3 h-3 rounded-full bg-green-500" }
            }
            div { class: "text-xs text-slate-400 font-mono", "{title}" }
            div { class: "w-10" }
        }
    }
}
