use include_dir::{Dir, include_dir};
use std::collections::HashMap;
use std::sync::OnceLock;

static CONTENT_ROOT: Dir = include_dir!("$CARGO_MANIFEST_DIR/content/etc/thoughts");

#[derive(Clone)]
pub struct Topic {
    pub slug: String,
    pub title: String,
    pub description: String,
    pub count: usize,
    pub color: &'static str,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum PostKind {
    Service,
    Conf,
    Log,
    Md,
}

#[derive(Clone)]
pub struct PostRef {
    pub slug: String,
    pub title: String,
    pub kind: PostKind,
}

#[derive(Clone)]
pub struct PostEntry {
    pub title: String,
    pub kind: PostKind,
    pub markdown: String,
}

pub fn topics() -> &'static Vec<Topic> {
    static TOPICS: OnceLock<Vec<Topic>> = OnceLock::new();
    TOPICS.get_or_init(|| {
        let mut list = CONTENT_ROOT
            .dirs()
            .filter(|dir| {
                let name = dir.path().file_name().unwrap_or_default().to_string_lossy();
                !name.starts_with('.')
            })
            .map(|dir| {
                let slug = dir.path().file_name().unwrap_or_default().to_string_lossy();
                let count = count_files(dir);
                let title = titleize(&slug);
                let (description, color) = topic_meta(&slug);

                Topic {
                    slug: slug.to_string(),
                    title,
                    description: description.to_string(),
                    count,
                    color,
                }
            })
            .collect::<Vec<_>>();

        list.sort_by(|a, b| a.slug.cmp(&b.slug));
        list
    })
}

pub fn posts_for(topic_slug: &str) -> Vec<PostRef> {
    let dir = match CONTENT_ROOT.get_dir(topic_slug) {
        Some(d) => d,
        None => return Vec::new(),
    };

    let mut posts = Vec::new();
    collect_posts(dir, topic_slug, &mut posts);
    posts.sort_by(|a, b| a.slug.cmp(&b.slug));
    posts
}

pub fn root_posts() -> Vec<PostRef> {
    let mut posts = Vec::new();
    for file in CONTENT_ROOT.files() {
        let name = file
            .path()
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();

        if !is_markdown_like(&name) {
            continue;
        }

        let (kind, base) = classify(&name);
        posts.push(PostRef {
            slug: base.clone(),
            title: titleize(&base),
            kind,
        });
    }

    posts.sort_by(|a, b| a.slug.cmp(&b.slug));
    posts
}

pub fn get_post(public_slug: &str) -> Option<PostEntry> {
    posts_index().get(public_slug).cloned()
}

pub fn search(query: &str) -> Vec<PostRef> {
    let q = query.trim().to_ascii_lowercase();
    if q.is_empty() {
        return Vec::new();
    }

    let mut out = Vec::new();
    for (slug, entry) in posts_index().iter() {
        let hay = format!("{} {}", slug, entry.title).to_ascii_lowercase();
        if hay.contains(&q) {
            out.push(PostRef {
                slug: slug.clone(),
                title: entry.title.clone(),
                kind: entry.kind,
            });
        }
    }

    out.sort_by(|a, b| a.slug.cmp(&b.slug));
    out
}

fn count_files(dir: &Dir) -> usize {
    let mut count = 0usize;

    for file in dir.files() {
        let name = file
            .path()
            .file_name()
            .unwrap_or_default()
            .to_string_lossy();
        if is_markdown_like(&name) {
            count += 1;
        }
    }

    for child in dir.dirs() {
        count += count_files(child);
    }

    count
}

fn collect_posts(dir: &Dir, topic_slug: &str, out: &mut Vec<PostRef>) {
    for file in dir.files() {
        let name = file
            .path()
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();

        if !is_markdown_like(&name) {
            continue;
        }

        let (kind, base) = classify(&name);
        let slug = format!("{topic_slug}/{base}");
        out.push(PostRef {
            slug,
            title: titleize(&base),
            kind,
        });
    }

    for child in dir.dirs() {
        let child_name = child
            .path()
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();

        if child_name.ends_with(".d") {
            let public_series = child_name.trim_end_matches(".d");
            for series_file in child.files() {
                let file_name = series_file
                    .path()
                    .file_name()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_string();

                if !is_markdown_like(&file_name) {
                    continue;
                }

                let base = strip_markdown_ext(&file_name);
                let slug = format!("{topic_slug}/{public_series}/{base}");
                out.push(PostRef {
                    slug,
                    title: titleize(base),
                    kind: PostKind::Md,
                });
            }
        } else {
            collect_posts(child, topic_slug, out);
        }
    }
}

fn classify(file_name: &str) -> (PostKind, String) {
    let base = strip_markdown_ext(file_name);

    if let Some(stem) = base.strip_suffix(".service") {
        return (PostKind::Service, stem.to_string());
    }
    if let Some(stem) = base.strip_suffix(".conf") {
        return (PostKind::Conf, stem.to_string());
    }
    if let Some(stem) = base.strip_suffix(".log") {
        return (PostKind::Log, stem.to_string());
    }

    (PostKind::Md, base.to_string())
}

fn posts_index() -> &'static HashMap<String, PostEntry> {
    static INDEX: OnceLock<HashMap<String, PostEntry>> = OnceLock::new();
    INDEX.get_or_init(build_posts_index)
}

fn build_posts_index() -> HashMap<String, PostEntry> {
    let mut out = HashMap::new();

    for file in CONTENT_ROOT.files() {
        let name = file
            .path()
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();

        if !is_markdown_like(&name) {
            continue;
        }

        let (kind, base) = classify(&name);
        let markdown = std::str::from_utf8(file.contents())
            .unwrap_or_default()
            .to_string();

        out.insert(
            base.clone(),
            PostEntry {
                title: titleize(&base),
                kind,
                markdown,
            },
        );
    }

    for topic in topics().iter() {
        if let Some(dir) = CONTENT_ROOT.get_dir(&topic.slug) {
            index_topic(dir, &topic.slug, &mut out);
        }
    }

    out
}

fn index_topic(dir: &Dir, topic_slug: &str, out: &mut HashMap<String, PostEntry>) {
    for file in dir.files() {
        let name = file
            .path()
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();

        if !is_markdown_like(&name) {
            continue;
        }

        let (kind, base) = classify(&name);
        let slug = format!("{topic_slug}/{base}");
        let markdown = std::str::from_utf8(file.contents())
            .unwrap_or_default()
            .to_string();

        out.insert(
            slug.clone(),
            PostEntry {
                title: titleize(&base),
                kind,
                markdown,
            },
        );
    }

    for child in dir.dirs() {
        let child_name = child
            .path()
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();

        if child_name.ends_with(".d") {
            let public_series = child_name.trim_end_matches(".d");
            for series_file in child.files() {
                let file_name = series_file
                    .path()
                    .file_name()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_string();

                if !is_markdown_like(&file_name) {
                    continue;
                }

                let base = strip_markdown_ext(&file_name);
                let slug = format!("{topic_slug}/{public_series}/{base}");
                let markdown = std::str::from_utf8(series_file.contents())
                    .unwrap_or_default()
                    .to_string();

                out.insert(
                    slug.clone(),
                    PostEntry {
                        title: titleize(base),
                        kind: PostKind::Md,
                        markdown,
                    },
                );
            }
        } else {
            index_topic(child, topic_slug, out);
        }
    }
}

fn strip_markdown_ext(file_name: &str) -> &str {
    file_name
        .strip_suffix(".mdx")
        .or_else(|| file_name.strip_suffix(".md"))
        .unwrap_or(file_name)
}

fn is_markdown_like(file_name: &str) -> bool {
    file_name.ends_with(".md") || file_name.ends_with(".mdx")
}

fn titleize(slug: &str) -> String {
    let mut out = String::new();
    for (i, part) in slug.split(['-', '_']).enumerate() {
        if part.is_empty() {
            continue;
        }
        if i > 0 {
            out.push(' ');
        }
        let mut chars = part.chars();
        if let Some(first) = chars.next() {
            out.push_str(&first.to_uppercase().to_string());
            out.push_str(chars.as_str());
        }
    }
    if out.is_empty() {
        slug.to_string()
    } else {
        out
    }
}

pub fn titleize_public(slug: &str) -> String {
    titleize(slug)
}

fn topic_meta(slug: &str) -> (&'static str, &'static str) {
    match slug {
        "platform" => (
            "Building internal developer platforms as products. K8s, IDPs, and paved roads.",
            "green",
        ),
        "system-design" => (
            "Consistency, availability, partition tolerance, and the headaches in between.",
            "blue",
        ),
        "agents" => (
            "Autonomous agents, orchestration, and the operational reality behind AI.",
            "purple",
        ),
        "mlops" => (
            "Pipelines, training, inference scaling, and the boring parts worth automating.",
            "purple",
        ),
        "philosophy" => (
            "Principles, mental models, and why reliability is a human concern.",
            "green",
        ),
        _ => ("A domain of thought under ThinkOS.", "green"),
    }
}
