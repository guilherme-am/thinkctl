pub fn split_frontmatter(input: &str) -> (Option<&str>, &str) {
    let trimmed = input.strip_prefix("---\n");
    let Some(without_open) = trimmed else {
        return (None, input);
    };

    let Some(end_idx) = without_open.find("\n---\n") else {
        return (None, input);
    };

    let frontmatter = &without_open[..end_idx];
    let body = &without_open[end_idx + "\n---\n".len()..];
    (Some(frontmatter), body)
}

pub fn frontmatter_title(frontmatter: &str) -> Option<String> {
    for line in frontmatter.lines() {
        let line = line.trim();
        let Some(rest) = line.strip_prefix("title:") else {
            continue;
        };

        let value = rest.trim();
        if value.is_empty() {
            continue;
        }

        let unquoted = value
            .strip_prefix('"')
            .and_then(|s| s.strip_suffix('"'))
            .unwrap_or(value);

        return Some(unquoted.to_string());
    }
    None
}

pub fn frontmatter_scalar(frontmatter: &str, key: &str) -> Option<String> {
    for line in frontmatter.lines() {
        let line = line.trim();
        let Some(rest) = line.strip_prefix(key) else {
            continue;
        };
        let Some(rest) = rest.strip_prefix(':') else {
            continue;
        };

        let value = rest.trim();
        if value.is_empty() {
            continue;
        }

        let unquoted = value
            .strip_prefix('"')
            .and_then(|s| s.strip_suffix('"'))
            .unwrap_or(value);

        return Some(unquoted.to_string());
    }
    None
}

pub fn frontmatter_list(frontmatter: &str, key: &str) -> Vec<String> {
    let Some(raw) = frontmatter_scalar(frontmatter, key) else {
        return Vec::new();
    };

    let inner = raw.trim();
    let inner = inner
        .strip_prefix('[')
        .and_then(|s| s.strip_suffix(']'))
        .unwrap_or(inner);

    inner
        .split(',')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| {
            s.strip_prefix('"')
                .and_then(|x| x.strip_suffix('"'))
                .unwrap_or(s)
        })
        .map(|s| s.to_string())
        .collect()
}

pub fn markdown_to_html(markdown: &str) -> String {
    use pulldown_cmark::{Options, Parser, html};

    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);
    options.insert(Options::ENABLE_FOOTNOTES);

    let parser = Parser::new_ext(markdown, options);

    let mut out = String::new();
    html::push_html(&mut out, parser);
    out
}

#[derive(Clone)]
pub struct TocItem {
    pub id: String,
    pub title: String,
    pub level: u8,
}

pub struct RenderedMarkdown {
    pub html: String,
    pub toc: Vec<TocItem>,
}

pub fn render_markdown(markdown: &str) -> RenderedMarkdown {
    let raw = markdown_to_html(markdown);
    let (with_ids, toc) = inject_heading_ids(&raw);
    let with_code = wrap_codeblocks(&with_ids);
    let with_admonitions = transform_admonitions(&with_code);

    RenderedMarkdown {
        html: with_admonitions,
        toc,
    }
}

fn inject_heading_ids(html: &str) -> (String, Vec<TocItem>) {
    use std::collections::HashMap;

    let mut out = String::with_capacity(html.len() + html.len() / 20);
    let mut toc = Vec::new();
    let mut seen: HashMap<String, usize> = HashMap::new();

    let mut i = 0usize;
    while i < html.len() {
        let h2 = html[i..].find("<h2>");
        let h3 = html[i..].find("<h3>");

        let (level, rel_start) = match (h2, h3) {
            (Some(a), Some(b)) => {
                if a <= b {
                    (2u8, a)
                } else {
                    (3u8, b)
                }
            }
            (Some(a), None) => (2u8, a),
            (None, Some(b)) => (3u8, b),
            (None, None) => {
                out.push_str(&html[i..]);
                break;
            }
        };

        let start = i + rel_start;
        out.push_str(&html[i..start]);

        let open_tag = if level == 2 { "<h2>" } else { "<h3>" };
        let close_tag = if level == 2 { "</h2>" } else { "</h3>" };

        let inner_start = start + open_tag.len();
        let Some(close_rel) = html[inner_start..].find(close_tag) else {
            out.push_str(&html[start..]);
            break;
        };
        let inner_end = inner_start + close_rel;
        let inner = &html[inner_start..inner_end];

        let title = strip_tags(inner).trim().to_string();
        let mut id = slugify(&title);
        if id.is_empty() {
            id = format!("section-{}", toc.len() + 1);
        }
        if let Some(n) = seen.get_mut(&id) {
            *n += 1;
            id = format!("{}-{}", id, *n);
        } else {
            seen.insert(id.clone(), 1);
        }

        toc.push(TocItem {
            id: id.clone(),
            title: title.clone(),
            level,
        });

        out.push('<');
        out.push('h');
        out.push(char::from_digit(level as u32, 10).unwrap_or('2'));
        out.push_str(" id=\"");
        out.push_str(&id);
        out.push_str("\">");
        out.push_str(inner);
        out.push_str(close_tag);

        i = inner_end + close_tag.len();
    }

    (out, toc)
}

fn strip_tags(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    let mut in_tag = false;
    for ch in input.chars() {
        match ch {
            '<' => in_tag = true,
            '>' => in_tag = false,
            _ if !in_tag => out.push(ch),
            _ => {}
        }
    }
    out
}

fn slugify(input: &str) -> String {
    let mut out = String::new();
    let mut last_dash = false;
    for ch in input.chars() {
        let c = ch.to_ascii_lowercase();
        if c.is_ascii_alphanumeric() {
            out.push(c);
            last_dash = false;
            continue;
        }
        if (c.is_whitespace() || c == '-' || c == '_' || c == '/') && !out.is_empty() && !last_dash
        {
            out.push('-');
            last_dash = true;
        }
    }
    while out.ends_with('-') {
        out.pop();
    }
    out
}

fn wrap_codeblocks(html: &str) -> String {
    let mut out = String::with_capacity(html.len() + html.len() / 10);
    let mut i = 0usize;

    while let Some(rel) = html[i..].find("<pre><code") {
        let start = i + rel;
        out.push_str(&html[i..start]);

        let Some(end_rel) = html[start..].find("</pre>") else {
            out.push_str(&html[start..]);
            return out;
        };
        let end = start + end_rel + "</pre>".len();
        let block = &html[start..end];

        out.push_str(
            "<div class=\"codeblock\"><button class=\"codecopy\" type=\"button\">copy</button>",
        );
        out.push_str(block);
        out.push_str("</div>");

        i = end;
    }

    out.push_str(&html[i..]);
    out
}

fn transform_admonitions(html: &str) -> String {
    let mut out = String::with_capacity(html.len());
    let mut i = 0usize;

    while let Some(rel) = html[i..].find("<blockquote>") {
        let start = i + rel;
        out.push_str(&html[i..start]);

        let Some(end_rel) = html[start..].find("</blockquote>") else {
            out.push_str(&html[start..]);
            return out;
        };
        let end = start + end_rel + "</blockquote>".len();
        let block = &html[start + "<blockquote>".len()..end - "</blockquote>".len()];
        let trimmed = block.trim_start();

        if let Some(kind) = extract_callout_kind(trimmed) {
            let body = strip_first_callout_paragraph(trimmed);
            out.push_str("<div class=\"admonition admonition-");
            out.push_str(kind);
            out.push_str("\"><div class=\"admonition-title\">");
            out.push_str(kind);
            out.push_str("</div><div class=\"admonition-body\">");
            out.push_str(body);
            out.push_str("</div></div>");
        } else {
            out.push_str("<blockquote>");
            out.push_str(block);
            out.push_str("</blockquote>");
        }

        i = end;
    }

    out.push_str(&html[i..]);
    out
}

fn extract_callout_kind(blockquote_inner: &str) -> Option<&'static str> {
    let prefixes = [
        ("[!NOTE]", "note"),
        ("[!TIP]", "tip"),
        ("[!IMPORTANT]", "important"),
        ("[!WARNING]", "warning"),
        ("[!CAUTION]", "caution"),
    ];

    let p_start = blockquote_inner.find("<p>")?;
    let p_end = blockquote_inner[p_start + 3..].find("</p>")?;

    let first = &blockquote_inner[p_start + 3..p_start + 3 + p_end];
    let first = first.trim();
    for (needle, kind) in prefixes {
        if first == needle {
            return Some(kind);
        }
    }
    None
}

fn strip_first_callout_paragraph(blockquote_inner: &str) -> &str {
    let Some(p_start) = blockquote_inner.find("<p>") else {
        return blockquote_inner;
    };
    let Some(p_end) = blockquote_inner[p_start + 3..].find("</p>") else {
        return blockquote_inner;
    };
    &blockquote_inner[p_start + 3 + p_end + "</p>".len()..]
}
