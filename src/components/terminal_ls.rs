use crate::components::themes;
use crate::ui::terminal_ls::TerminalLsRowVm;
use dioxus::prelude::*;

#[component]
pub fn TerminalLsRow(row: TerminalLsRowVm) -> Element {
    let kind_icon = themes::kind_icon(row.kind);
    let kind_color = themes::kind_color(row.kind);

    rsx! {
        a {
            href: "{row.href}",
            style: "text-decoration: none;",
            class: "ls-row",
            span { class: "ls-col ls-perms text-slate-500", "{row.perms}" }
            span { class: "ls-col ls-links text-slate-500", "{row.links}" }
            span { class: "ls-col ls-owner text-slate-500", "{row.owner}" }
            span { class: "ls-col ls-group text-slate-500", "{row.group}" }
            span { class: "ls-col ls-size text-slate-500", "{row.size}" }
            span { class: "ls-col ls-stamp text-slate-500", "{row.stamp}" }
            span { class: "ls-col ls-name",
                span { class: format!("text-xs {}", kind_color), "{kind_icon}" }
                span { class: "ls-name-text text-slate-300", "{row.name}" }
            }
        }
    }
}
