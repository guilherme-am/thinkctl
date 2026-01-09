# Agent Guide

## Rules and Conventions

- Canonical rules: `.cursor/rules/rust-rules.mdc` and `.kiro-cli/rules/rust-rules.mdc`
- Follow Rust idioms: ownership, borrowing, error handling with `Result<T, E>`
- Architectural hygiene: SOLID, DRY, composition over inheritance
- Keep functions focused and modules small
- Use `cargo fmt` and `cargo clippy` before committing

### Before Writing ANY Code

1. **Create specification document** in your AI tool's specs folder:
   - Amazon Q: `.amazonq/specs/[feature-name]/spec.md`
   - Cursor: `.cursor/specs/[feature-name]/spec.md`
   - Kiro: `.kiro-cli/specs/[feature-name]/spec.md`

2. **Use spec.md as living document** - Update it as you code, track progress, notes, decisions
   - Avoid creating multiple files like `SUMMARY.md`, `PLAN.md`, `NOTES.md`
   - Keep it simple: one `spec.md` per feature

3. **Fill key sections:**
   - Overview (problem, goal)
   - Architecture (components, data flow)
   - Implementation notes (as you code)
   - Testing approach

## Project Structure

```bash
thinkctl/
├── Cargo.toml
├── src/
│   ├── main.rs
│   ├── app.rs
│   ├── routes.rs
│   ├── components/
│   └── pages/
├── content/
│   └── etc/
│       └── thoughts/
├── public/
└── .github/
    └── workflows/
```

## Tooling

- Format: `cargo fmt --all`
- Lint: `cargo clippy --all-targets --all-features -- -D warnings`
- Build: `cargo build --release`
- WASM: `dx build --release` (Dioxus)
- After writing or editing markdown, run markdownlint on content/specs/skills.
  See `.skills/markdownlint/SKILL.md`.

## Content Conventions

## /etc/thoughts — Canonical Content & URL Specification

### Core concept

`/etc/thoughts/` represents the configuration directory of ThinkOS.

### Golden rule

- Directory name defines the domain of thought.
- File extension defines the intent of the content.

Folders answer “what is this about?”. Extensions answer “how should this be read?”.

### Directory semantics (topics)

Directories under `/etc/thoughts/` represent domains/themes/areas of thinking
(e.g. `agents/`, `mlops/`, `platform/`, `system-design/`, `philosophy/`).

### File semantics (intent)

- `.service.md`: architecture/system design/workflows/processes (system-level, reusable concepts)
- `.conf.md`: principles/opinions/best practices/mental models (declarative configuration of beliefs)
- `.log.md`: narrative incidents/debugging/lessons learned (temporal, reflective)
- `.d/`: series directories (multi-part deep dives, ordered files)
- `.md`: neutral/default when it doesn’t fit the above (exception, not default)

### URL mapping rules

Source file:

`/etc/thoughts/agents/plan-and-execute.service.md`

Public URL:

`/etc/thoughts/agents/plan-and-execute`

Extensions are semantic and not visible in URLs.

### Required system units

These must always exist at the root of `/etc/thoughts/`:

- `welcome.service.md`
- `whoami.conf.md`
- `purpose.conf.md`

### Folder Structure

```bash
/etc/thoughts/
├── agents/
├── mlops/
├── platform/
├── system-design/
└── philosophy/
```

### File Extensions

- `.service.md` - Architecture, design, system-level posts
- `.log.md` - Experience, debugging, lessons learned
- `.conf.md` - Principles, guidelines, best practices
- `.d/` - Multi-part series (e.g., `bgp.d/00-overview.md`)
- `.md` - Plain posts

## Where to Look

- Components: `src/components/`
- Pages: `src/pages/`
- Routes: `src/routes.rs`
- Content: `content/etc/thoughts/`
- Static assets: `public/`
- CI/CD: `.github/workflows/`

## Markdown Processing

- Frontmatter (optional):

  ```yaml
  ---
  title: "Post Title"
  type: service
  tags: [agents, ai]
  ---
  ```
  
- Parsed at build time
- Supports code blocks, future: diagrams, callouts

## Agent Skills

This project uses [Agent Skills](https://github.com/anthropics/agentskills) - an open standard for
packaging domain expertise into composable resources.

### Available Skills

Skills are located in `.skills/` directory:

- `thinkos-content-creator` - Guide for creating ThinkOS content with proper file extensions and structure

### Creating New Skills

```bash
# Initialize a new skill (requires skills repo cloned locally)
cd /path/to/skills
python3 skills/skill-creator/scripts/init_skill.py <skill-name> --path /path/to/project/.skills
```

### Using Skills

When creating content or working with the `/etc/thoughts` system, reference the skill:

```bash
# Validate a skill (requires agentskills repo cloned locally)
cd /path/to/agentskills/skills-ref
source .venv/bin/activate
skills-ref validate /path/to/skill

# Generate prompt XML for agents
skills-ref to-prompt .skills/thinkos-content-creator
```

Skills use progressive disclosure - load only `SKILL.md` initially, then reference linked files as needed.

## Deployment

- Target: GitHub Pages
- Domain: `thinkctl.dev`
- Static output from WASM build
- CI/CD via GitHub Actions
