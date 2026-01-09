use dioxus::prelude::*;

#[component]
pub fn Footer() -> Element {
    rsx! {
        footer { class: "border-t border-slate-800 bg-slate-950",
            div { class: "max-w-6xl mx-auto px-6 py-10 flex flex-col md:flex-row items-start md:items-center justify-between gap-6",
                div {
                    div { class: "font-mono text-slate-200",
                        "> thinkctl.dev"
                    }
                    div { class: "text-slate-500 text-sm mt-2",
                        "Start, stop, and restart your thinking."
                    }
                }
                div { class: "flex flex-wrap gap-4 font-mono text-sm",
                    a {
                        class: "text-slate-400 hover:text-green-400 transition-colors",
                        style: "text-decoration: none;",
                        href: "https://github.com/guilherme-am/thinkctl",
                        target: "_blank",
                        "Repo"
                    }
                    a {
                        class: "text-slate-400 hover:text-green-400 transition-colors",
                        style: "text-decoration: none;",
                        href: "https://github.com/guilherme-am/thinkctl/stargazers",
                        target: "_blank",
                        "★ stars"
                    }
                    a {
                        class: "text-slate-400 hover:text-green-400 transition-colors",
                        style: "text-decoration: none;",
                        href: "https://github.com/guilherme-am",
                        target: "_blank",
                        "GitHub"
                    }
                    a {
                        class: "text-slate-400 hover:text-green-400 transition-colors",
                        style: "text-decoration: none;",
                        href: "https://www.linkedin.com/in/guilhermeamoreira/",
                        target: "_blank",
                        "LinkedIn"
                    }
                    a {
                        class: "text-slate-400 hover:text-green-400 transition-colors",
                        style: "text-decoration: none;",
                        href: "mailto:guilherme.amoreira96@gmail.com",
                        "Email"
                    }
                }
            }
        }
    }
}
