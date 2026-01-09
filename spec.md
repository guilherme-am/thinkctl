# thinkctl.dev — Technical Specification

## Project Overview

**thinkctl.dev** is a personal technical blog and experimentation platform, designed as a fictional operating system called **ThinkOS**.

The site presents ideas, experiences, and technical content as system components (services, logs, configs), using operating-system metaphors inspired by Linux, DevOps, platform engineering, MLOps, and AI agents.

The goal is to create:
- a memorable, engineer-native reading experience
- a long-term personal knowledge base
- a playground to experiment with Rust, WASM, UI systems, and agent-oriented thinking

---

## Branding

### Name
**thinkctl**

### Domain
- Primary: `thinkctl.dev`
- Blog root: `/etc/thoughts`

### Slogan
> **Start, stop, and restart your thinking.**

---

## High-Level Architecture

- **Language:** Rust
- **Frontend framework:** Dioxus (Web / WASM)
- **Rendering:** Static site generation (SSG-style)
- **Hosting:** GitHub Pages
- **CI/CD:** GitHub Actions
- **Content format:** Markdown
- **Design metaphor:** Terminal / Linux filesystem / systemd

---

## UX & UI Concept

### Landing Page (`/`)

The landing page simulates a terminal booting ThinkOS.

#### Behavior
- Animated terminal loop:
```
thinkctl start
thinkctl status
thinkctl restart
thinkctl stop
```
- Commands appear sequentially, with a blinking cursor.
- After the loop, a simulated `tree` command is rendered:

```
$ tree /etc/thoughts
```

- Output shows the blog structure (directories are clickable).

#### Purpose
- Immediately communicate theme and personality
- Invite users to explore content as a filesystem

---

## Content Model

### Root Content Path

```
/etc/thoughts
```

All blog content lives under this namespace.

---

## Folder Naming Convention (Topics)

Folders represent **domains of thought**, not fixed system directories.

Examples (non-exhaustive):

```
/etc/thoughts/
├── agents/
├── mlops/
├── platform/
├── system-design/
├── kernel/
├── networking/
├── philosophy/
```

- Folder names define **what the content is about**
- They are flexible and can evolve over time
- No fixed mandatory folders (e.g. not always `logs/`, `systemd/`, etc.)

---

## File Naming Convention (Post Semantics)

The **file extension encodes the tone and intent** of the post.

### `.service`
**Architectural, conceptual, or system-level design posts**

Used for:
- system architecture
- workflows
- platform design
- agent orchestration
- MLOps pipelines

Examples:
```
agents/plan-and-execute.service.md
mlops/experiment-tracking.service.md
platform/k8s-operator-design.service.md
```

---

### `.log`
**Experience-driven, narrative, or debugging posts**

Used for:
- incidents
- failures
- lessons learned
- on-call stories
- personal reflections

Examples:
```
platform/broke-prod-again.log.md
kernel/chasing-a-race-condition.log.md
mlops/latency-nightmare.log.md
```

---

### `.conf`
**Opinionated configuration, principles, or guidelines**

Used for:
- best practices
- mental models
- architectural beliefs
- philosophy

Examples:
```
system-design/idempotency.conf.md
philosophy/automation.conf.md
agents/prompting.conf.md
```

---

### `.d/` directories
**Collections or series**, inspired by `*.d` drop-in directories.

Used for:
- multi-part deep dives
- progressive explanations

Example:
```
networking/bgp.d/
├── 00-overview.md
├── 10-problems.md
└── 20-lessons.md
```

---

### `.md`
Plain markdown posts when no semantic wrapper is needed.

---

## Example Content Tree

```
/etc/thoughts
├── agents/
│   ├── agent-memory.service.md
│   ├── reasoning-vs-execution.log.md
│   └── prompting.conf.md
│
├── mlops/
│   ├── pipelines.service.md
│   ├── feature-stores.conf.md
│   └── training-failures.log.md
│
├── platform/
│   ├── terraform-hell.log.md
│   ├── kubernetes-patterns.service.md
│   └── reliability.conf.md
│
├── system-design/
│   ├── caching-strategies.service.md
│   └── retry-storms.log.md
│
└── philosophy/
    ├── what-is-devops.conf.md
    └── learning-curves.log.md
```

---

## Repository Structure

```
thinkctl/
├── Cargo.toml
├── Cargo.lock
├── src/
│   ├── main.rs
│   ├── app.rs
│   ├── routes.rs
│   ├── components/
│   │   ├── terminal.rs
│   │   ├── tree.rs
│   │   └── layout.rs
│   └── pages/
│       ├── home.rs
│       └── etc_thoughts.rs
├── content/
│   └── etc/
│       └── thoughts/   # markdown content lives here
├── public/
│   ├── favicon.svg
│   └── assets/
├── .github/
│   └── workflows/
│       └── deploy.yml
├── rustfmt.toml
├── clippy.toml
└── spec.md
```

---

## Markdown Processing

- Markdown files are parsed at build time
- Frontmatter (optional):
  ```yaml
  ---
  title: "Plan and Execute Agents"
  type: service
  tags: [agents, ai, architecture]
  ---
  ```
- Metadata is used for navigation, titles, and filtering
- Rendering supports:
  - code blocks
  - diagrams (future)
  - callouts (future)

---

## Dioxus Usage

- `dioxus-web` target
- `dioxus-router` for routing
- WASM build output served statically
- Terminal animation implemented as a Dioxus component
- Tree view rendered from parsed content directory

---

## Tooling & Quality

### Rust Toolchain

- Stable Rust
- `rustfmt` for formatting
- `clippy` for linting
- `cargo audit` (optional, future)
- `cargo deny` (optional, future)

### Formatting

```bash
cargo fmt --all
```

### Linting

```bash
cargo clippy --all-targets --all-features -- -D warnings
```

---

## CI/CD (GitHub Actions)

### Workflow Goals

- Build site
- Lint and format check
- Deploy to GitHub Pages

### Pipeline Steps

1. Checkout repo
2. Install Rust toolchain
3. Install WASM target
4. Run `cargo fmt --check`
5. Run `cargo clippy`
6. Build WASM bundle
7. Deploy static output to `gh-pages` branch

### Trigger

- On push to `main`

---

## Hosting

- GitHub Pages
- Custom domain: `thinkctl.dev`
- HTTPS enforced
- Static assets only

---

## Non-Goals (for now)

- Comments
- Auth
- Dynamic backend
- Databases
- Analytics (optional later)

---

## Future Ideas

- Keyboard navigation (`j/k`, `/`)
- Fake `man thinkctl` pages
- Search as `grep`
- RSS as `journalctl`
- Dark/light terminal themes
- Agent-generated summaries
- CLI tool that mirrors blog content

---

## Summary

**thinkctl.dev** is not just a blog.

It is:
- a personal operating system
- a long-term knowledge archive
- a playground for Rust + UI + systems thinking
- a narrative device for platform engineering, MLOps, and AI agents

> *Start, stop, and restart your thinking.*
