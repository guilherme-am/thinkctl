use dioxus::prelude::Key;
use dioxus::prelude::*;
use gloo_timers::future::TimeoutFuture;
use wasm_bindgen_futures::spawn_local;
use web_sys::wasm_bindgen::JsCast;

#[component]
pub fn Terminal(on_complete: Option<EventHandler<()>>) -> Element {
    #[derive(Clone, Copy, PartialEq)]
    enum LineKind {
        Header,
        Cmd,
        Output,
        Success,
    }

    #[derive(Clone, Copy, PartialEq)]
    struct ScriptStep {
        kind: LineKind,
        text: &'static str,
        pre_delay_ms: u32,
        type_ms: u32,
    }

    #[derive(Clone, PartialEq)]
    struct RenderLine {
        kind: LineKind,
        text: String,
        typing: bool,
    }

    let mut started = use_signal(|| false);
    let mut lines = use_signal(Vec::<RenderLine>::new);
    let done = use_signal(|| false);
    let mut input = use_signal(|| "thinkctl ".to_string());
    let terminal_id = "terminal-body";

    use_effect(move || {
        if *started.read() {
            return;
        }

        started.set(true);
        let lines = lines;
        let on_complete = on_complete;
        let mut done = done;

        spawn_local(async move {
            let mut lines = lines;
            let script = [
                ScriptStep {
                    kind: LineKind::Header,
                    text: "thinkctl.dev",
                    pre_delay_ms: 400,
                    type_ms: 0,
                },
                ScriptStep {
                    kind: LineKind::Header,
                    text: "──────────────────────────────",
                    pre_delay_ms: 120,
                    type_ms: 0,
                },
                ScriptStep {
                    kind: LineKind::Cmd,
                    text: "thinkctl start",
                    pre_delay_ms: 450,
                    type_ms: 28,
                },
                ScriptStep {
                    kind: LineKind::Output,
                    text: "Starting ThinkOS...",
                    pre_delay_ms: 250,
                    type_ms: 0,
                },
                ScriptStep {
                    kind: LineKind::Cmd,
                    text: "thinkctl load inspirations",
                    pre_delay_ms: 450,
                    type_ms: 24,
                },
                ScriptStep {
                    kind: LineKind::Success,
                    text: "platform-engineering",
                    pre_delay_ms: 140,
                    type_ms: 0,
                },
                ScriptStep {
                    kind: LineKind::Success,
                    text: "distributed-systems",
                    pre_delay_ms: 120,
                    type_ms: 0,
                },
                ScriptStep {
                    kind: LineKind::Success,
                    text: "reliability",
                    pre_delay_ms: 120,
                    type_ms: 0,
                },
                ScriptStep {
                    kind: LineKind::Success,
                    text: "ai-agents",
                    pre_delay_ms: 120,
                    type_ms: 0,
                },
                ScriptStep {
                    kind: LineKind::Success,
                    text: "sustainability",
                    pre_delay_ms: 120,
                    type_ms: 0,
                },
                ScriptStep {
                    kind: LineKind::Cmd,
                    text: "thinkctl tech-stack.service",
                    pre_delay_ms: 420,
                    type_ms: 22,
                },
                ScriptStep {
                    kind: LineKind::Output,
                    text: "cloud: aws gcp azure",
                    pre_delay_ms: 110,
                    type_ms: 0,
                },
                ScriptStep {
                    kind: LineKind::Output,
                    text: "containers: kubernetes openshift",
                    pre_delay_ms: 110,
                    type_ms: 0,
                },
                ScriptStep {
                    kind: LineKind::Output,
                    text: "iac: aws-cdk terraform ansible githubactions gitlab",
                    pre_delay_ms: 110,
                    type_ms: 0,
                },
                ScriptStep {
                    kind: LineKind::Output,
                    text: "observability: prometheus grafana",
                    pre_delay_ms: 110,
                    type_ms: 0,
                },
                ScriptStep {
                    kind: LineKind::Output,
                    text: "ml: mlflow kubeflow",
                    pre_delay_ms: 110,
                    type_ms: 0,
                },
                ScriptStep {
                    kind: LineKind::Cmd,
                    text: "thinkctl whoami",
                    pre_delay_ms: 420,
                    type_ms: 22,
                },
                ScriptStep {
                    kind: LineKind::Output,
                    text: "user: guilherme-am",
                    pre_delay_ms: 120,
                    type_ms: 0,
                },
                ScriptStep {
                    kind: LineKind::Output,
                    text: "role: platform & devops engineer",
                    pre_delay_ms: 120,
                    type_ms: 0,
                },
                ScriptStep {
                    kind: LineKind::Output,
                    text: "focus: systems, reliability, ai agents, tooling",
                    pre_delay_ms: 120,
                    type_ms: 0,
                },
                ScriptStep {
                    kind: LineKind::Output,
                    text: "mission: build systems that help the world communicate better and sustainably",
                    pre_delay_ms: 120,
                    type_ms: 0,
                },
                ScriptStep {
                    kind: LineKind::Cmd,
                    text: "thinkctl expertise",
                    pre_delay_ms: 420,
                    type_ms: 22,
                },
                ScriptStep {
                    kind: LineKind::Output,
                    text: "core: cloud & containerized systems (oci, docker, podman/rootless, containerd)",
                    pre_delay_ms: 120,
                    type_ms: 0,
                },
                ScriptStep {
                    kind: LineKind::Output,
                    text: "platform: kubernetes (openshift, gke, eks) + helm + gitops (argo cd)",
                    pre_delay_ms: 120,
                    type_ms: 0,
                },
                ScriptStep {
                    kind: LineKind::Output,
                    text: "ops: slos/slis, incident response, observability (prometheus/grafana/otel)",
                    pre_delay_ms: 120,
                    type_ms: 0,
                },
                ScriptStep {
                    kind: LineKind::Cmd,
                    text: "thinkctl certifications",
                    pre_delay_ms: 420,
                    type_ms: 22,
                },
                ScriptStep {
                    kind: LineKind::Output,
                    text: "aws: cloud consultant specialist; cloudops engineer",
                    pre_delay_ms: 120,
                    type_ms: 0,
                },
                ScriptStep {
                    kind: LineKind::Output,
                    text: "google: cloud architect",
                    pre_delay_ms: 120,
                    type_ms: 0,
                },
                ScriptStep {
                    kind: LineKind::Output,
                    text: "nokia: bell labs 5g distributed cloud professional",
                    pre_delay_ms: 120,
                    type_ms: 0,
                },
                ScriptStep {
                    kind: LineKind::Cmd,
                    text: "tree /etc/thoughts",
                    pre_delay_ms: 420,
                    type_ms: 22,
                },
                ScriptStep {
                    kind: LineKind::Output,
                    text: "/etc/thoughts",
                    pre_delay_ms: 120,
                    type_ms: 0,
                },
                ScriptStep {
                    kind: LineKind::Output,
                    text: "├── agents/",
                    pre_delay_ms: 80,
                    type_ms: 0,
                },
                ScriptStep {
                    kind: LineKind::Output,
                    text: "├── mlops/",
                    pre_delay_ms: 80,
                    type_ms: 0,
                },
                ScriptStep {
                    kind: LineKind::Output,
                    text: "├── platform/",
                    pre_delay_ms: 80,
                    type_ms: 0,
                },
                ScriptStep {
                    kind: LineKind::Output,
                    text: "├── system-design/",
                    pre_delay_ms: 80,
                    type_ms: 0,
                },
                ScriptStep {
                    kind: LineKind::Output,
                    text: "└── philosophy/",
                    pre_delay_ms: 80,
                    type_ms: 0,
                },
            ];

            for step in script {
                TimeoutFuture::new(step.pre_delay_ms).await;

                if step.kind == LineKind::Cmd {
                    let idx = {
                        let mut guard = lines.write();
                        guard.push(RenderLine {
                            kind: LineKind::Cmd,
                            text: String::new(),
                            typing: true,
                        });
                        guard.len() - 1
                    };

                    for ch in step.text.chars() {
                        TimeoutFuture::new(step.type_ms).await;
                        let mut guard = lines.write();
                        if let Some(line) = guard.get_mut(idx) {
                            line.text.push(ch);
                        }
                    }

                    let mut guard = lines.write();
                    if let Some(line) = guard.get_mut(idx) {
                        line.typing = false;
                    }

                    continue;
                }

                lines.write().push(RenderLine {
                    kind: step.kind,
                    text: step.text.to_string(),
                    typing: false,
                });
            }

            done.set(true);

            if let Some(cb) = on_complete {
                cb.call(());
            }
        });
    });

    use_effect(move || {
        let _ = lines.read().len();
        let Some(win) = web_sys::window() else {
            return;
        };
        let Some(doc) = win.document() else {
            return;
        };
        let Some(el) = doc.get_element_by_id(terminal_id) else {
            return;
        };
        let html: web_sys::HtmlElement = match el.dyn_into() {
            Ok(v) => v,
            Err(_) => return,
        };
        html.set_scroll_top(html.scroll_height());
    });

    rsx! {
        div {
            class: "font-mono text-sm md:text-base text-slate-300 h-[420px] md:h-[500px] flex flex-col",

            div {
                id: "{terminal_id}",
                class: "p-6 flex-1 overflow-y-auto custom-scrollbar",

                for line in lines.read().iter() {
                    match line.kind {
                        LineKind::Header => rsx! {
                            div { class: "font-bold text-white mb-1", "{line.text}" }
                        },
                        LineKind::Cmd => rsx! {
            div { class: "mb-1",
                                span { class: "text-green-400 mr-2", "$" }
                                span { "{line.text}" }
                                if line.typing {
                                    span { class: "text-green-400 cursor-blink", "█" }
                                }
                            }
                        },
                        LineKind::Success => rsx! {
            div { class: "mb-1",
                span { class: "text-green-400", "✔ " }
                                "{line.text}"
                            }
                        },
                        LineKind::Output => rsx! {
                            div { class: "mb-1", "{line.text}" }
                        },
                    }
                }
            }

            if *done.read() {
                div { class: "border-t border-slate-700 bg-slate-950/70 px-4 py-3",
                    div { class: "flex items-center justify-between gap-6",
                        div { class: "text-slate-500 text-xs",
                            "try: thinkctl help | thinkctl start thoughts | thinkctl status experience"
                        }
                        a {
                            class: "text-slate-400 hover:text-green-400 transition-colors cursor-pointer font-mono text-xs flex items-center gap-2",
                            style: "text-decoration: none;",
                            href: "#main-content",
                            span { "Explore /etc/thoughts" }
                            span { class: "cursor-blink", "↓" }
                        }
                    }
                    div { class: "mt-2 flex items-center gap-2",
                        span { class: "text-green-400", "$" }
                        input {
                            class: "terminal-input",
                            value: "{input.read()}",
                            placeholder: "thinkctl help",
                            oninput: move |evt| {
                                input.set(evt.value());
                            },
                            onkeydown: move |evt| {
                                if evt.key() != Key::Enter {
                                    return;
                                }

                                let raw = input.read().trim().to_string();
                                if raw.is_empty() {
                                    return;
                                }

                                let cmd = raw.clone();
                                let mut out_lines = Vec::<RenderLine>::new();
                                let (actions, outputs) = interpret(&cmd);

                                out_lines.push(RenderLine {
                                    kind: LineKind::Cmd,
                                    text: cmd,
                                    typing: false,
                                });

                                for line in outputs {
                                    out_lines.push(RenderLine {
                                        kind: LineKind::Output,
                                        text: line,
                                        typing: false,
                                    });
                                }

                                lines.write().extend(out_lines);
                                input.set("thinkctl ".to_string());

                                for action in actions {
                                    run_action(action);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[derive(Clone, Copy)]
enum Action {
    ScrollTo(&'static str),
}

fn interpret(input: &str) -> (Vec<Action>, Vec<String>) {
    let raw = input.trim();
    let mut parts = raw.split_whitespace().collect::<Vec<_>>();
    if parts.first().copied() == Some("thinkctl") {
        parts.remove(0);
    }

    match parts.as_slice() {
        [] => (vec![], vec![]),
        ["help"] | ["--help"] => (
            vec![],
            vec![
                "commands:".to_string(),
                "  thinkctl start thoughts        -> jump to /etc/thoughts/themes".to_string(),
                "  thinkctl status experience     -> jump to whoami.conf".to_string(),
                "  thinkctl status stack          -> jump to expertise & stack".to_string(),
                "  thinkctl status certs          -> jump to certifications".to_string(),
                "  thinkctl help                  -> show this help".to_string(),
            ],
        ),
        ["start", "thoughts"] | ["thoughts"] => (
            vec![Action::ScrollTo("themes")],
            vec!["opening /etc/thoughts...".to_string()],
        ),
        ["status", "experience"] | ["experience"] => (
            vec![Action::ScrollTo("whoami")],
            vec!["jumping to whoami.conf...".to_string()],
        ),
        ["status", "stack"] => (
            vec![Action::ScrollTo("stack")],
            vec!["jumping to expertise & stack...".to_string()],
        ),
        ["status", "certs"] | ["status", "certifications"] => (
            vec![Action::ScrollTo("certs")],
            vec!["jumping to certifications...".to_string()],
        ),
        _ => (
            vec![],
            vec![
                format!("unknown command: {raw}"),
                "try: thinkctl help".to_string(),
            ],
        ),
    }
}

fn run_action(action: Action) {
    match action {
        Action::ScrollTo(id) => {
            let Some(win) = web_sys::window() else {
                return;
            };
            let Some(doc) = win.document() else {
                return;
            };
            let Some(el) = doc.get_element_by_id(id) else {
                return;
            };
            el.scroll_into_view();
        }
    }
}
