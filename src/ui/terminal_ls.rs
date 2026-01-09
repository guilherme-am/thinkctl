use crate::content_index::{PostKind, PostRef};
use crate::ui::markdown;

#[derive(Clone, PartialEq)]
pub struct TerminalLsRowVm {
    pub href: String,
    pub perms: String,
    pub links: String,
    pub owner: String,
    pub group: String,
    pub size: String,
    pub stamp: String,
    pub name: String,
    pub kind: PostKind,
}

pub fn row_for_post(post: &PostRef) -> TerminalLsRowVm {
    let href = format!("/etc/thoughts/{}", post.slug);

    let perms = match post.kind {
        PostKind::Service | PostKind::Conf | PostKind::Log | PostKind::Md => {
            "-rw-r--r--".to_string()
        }
    };

    let mut size = "--".to_string();
    let mut stamp = "--".to_string();
    if let Some(entry) = crate::content_index::get_post(&post.slug) {
        let (frontmatter, body) = markdown::split_frontmatter(&entry.markdown);
        size = body.trim().len().to_string();
        if let Some(fm) = frontmatter
            && let Some(ts) = markdown::frontmatter_timestamp(fm)
        {
            stamp = ts;
        }
    }

    TerminalLsRowVm {
        href,
        perms,
        links: "1".to_string(),
        owner: "thinkctl".to_string(),
        group: "staff".to_string(),
        size,
        stamp,
        name: unit_label(post),
        kind: post.kind,
    }
}

fn unit_label(post: &PostRef) -> String {
    let name = post.slug.split('/').next_back().unwrap_or(&post.slug);
    let ext = match post.kind {
        PostKind::Service => "service",
        PostKind::Conf => "conf",
        PostKind::Log => "log",
        PostKind::Md => "md",
    };
    format!("{name}.{ext}")
}
