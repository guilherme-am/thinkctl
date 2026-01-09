---
name: writer-voice
description: Apply the thinkctl.dev writer voice to new or existing posts; rewrite generic AI text into Guilherme’s tone. Use when drafting, rewriting or reviewing content under content/etc/thoughts.
license: Proprietary
compatibility: Filesystem-based agents (Cursor/Claude Code). Requires access to this repo; no network required.
metadata:
  author: guilherme-am
  version: "1.0"
---

# Writer Voice (thinkctl.dev)

## Canonical writer profile (source of truth)

Read:

- `content/etc/thoughts/.internal/writer.conf.md`

This file is intentionally not public-facing.

## When to use

- Drafting new posts under `content/etc/thoughts/**`
- Rewriting posts that feel generic, salesy, or “AI-ish”
- Keeping a series consistent (e.g. `docs.d/`)

## Output constraints

- Keep ThinkOS semantics:
  - Topic folders under `content/etc/thoughts/`
  - Semantic extensions (`.service/.conf/.log`) for intent
  - Series directories `*.d/` for sequences
- Avoid multiple H1 headings; prefer `##` and below (frontmatter carries the title).

## Rewrite checklist (anti-AI)

Before shipping a post, ensure it contains:

- Natural pronoun balance (`I` / `we` / `you`)
- “You” used gently (no blame, no commands)
- Humble learning orientation ("I'm learning too, what do you think?")
- Invitation to collaborate or contribute
- Sustainability focus (resource efficiency, reusability, modularity)
- Warm, soft tone (never aggressive or hard)
- Open source values (sharing, community, collaboration)
- Humility about what you don't know

## Preferred structure

- Context → Perspective → Invitation → Humility

## Phrase hygiene

Avoid:

- "The hard truth is..."
- "I know this from production..."
- Aggressive or confrontational language
- Self-exhibitionism
- "You should..." (use "we could..." or "maybe...")
- Competing or rivalry language

Prefer:

- Balanced `I / we / you` (avoid “we” in every paragraph)
- "Maybe this helps"
- "I'm still learning"
- "What do you think?"
- Warm, inviting language
- Questions over declarations
