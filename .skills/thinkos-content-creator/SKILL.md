---
name: thinkos-content-creator
description: Guide for creating content in the ThinkOS blog system. Use when creating new posts, understanding the /etc/thoughts directory structure, or working with .service.md, .conf.md, .log.md files.
---

# ThinkOS Content Creator

This skill teaches how to create content for the ThinkOS blog system,
which treats the website as an operating system where ideas are services,
stories are logs, and beliefs are configuration files.

## Core Concept

`/etc/thoughts/` represents the configuration directory of ThinkOS. The file system structure has semantic meaning.

## Golden Rule

- **Directory name** = domain of thought (what is this about?)
- **File extension** = intent of content (how should this be read?)

## Directory Structure (Topics)

```bash
content/etc/thoughts/
├── agents/          # AI agents, agentic systems
├── mlops/           # ML operations, model deployment
├── platform/        # Platform engineering, K8s, infrastructure
├── system-design/   # Distributed systems, architecture
└── philosophy/      # Engineering philosophy, principles
```

Create new directories for new domains as needed.

## File Extensions (Intent)

- **`.service.md`** - Architecture, system design, workflows, processes
  - System-level, reusable concepts
  - Example: `plan-and-execute.service.md`, `k8s-platform.service.md`

- **`.conf.md`** - Principles, opinions, best practices, mental models
  - Declarative configuration of beliefs
  - Example: `whoami.conf.md`, `reliability.conf.md`

- **`.log.md`** - Narrative incidents, debugging, lessons learned
  - Temporal, reflective, story-driven
  - Example: `broke-prod.log.md`, `agent-sre.log.md`

- **`.d/`** - Series directories for multi-part deep dives
  - Ordered files: `00-overview.md`, `01-setup.md`, etc.
  - Example: `bgp.d/`, `kubernetes-deep-dive.d/`

- **`.md`** - Neutral/default (use sparingly)
  - Only when content doesn't fit above categories

## Required System Units

These MUST exist at `/etc/thoughts/` root:

- `welcome.service.md` - Introduction to ThinkOS
- `whoami.conf.md` - Author bio/configuration
- `purpose.conf.md` - System purpose/mission

## URL Mapping

Extensions are semantic but NOT visible in URLs:

```bash
Source: /etc/thoughts/agents/plan-and-execute.service.md
URL:    /etc/thoughts/agents/plan-and-execute
```

## Frontmatter (Optional)

```yaml
---
title: "Post Title"
type: service
tags: [agents, ai, platform]
date: 2025-01-08
---
```

## Creating New Content

1. **Choose domain** - Which directory? (agents/, platform/, etc.)
2. **Choose intent** - Which extension? (.service, .conf, .log)
3. **Create file** - `content/etc/thoughts/{domain}/{name}.{ext}.md`
4. **Write content** - Markdown with optional frontmatter

## Examples

**Architecture post:**

```bash
content/etc/thoughts/platform/k8s-operator-pattern.service.md
```

**Principle post:**

```bash
content/etc/thoughts/philosophy/automate-yourself.conf.md
```

**Incident story:**

```bash
content/etc/thoughts/platform/deleted-ingress.log.md
```

**Multi-part series:**

```bash
content/etc/thoughts/system-design/cap-theorem.d/
├── 00-overview.md
├── 01-consistency.md
├── 02-availability.md
└── 03-partition-tolerance.md
```

## Best Practices

- Keep posts focused and concise
- Use code blocks for technical examples
- Link between related posts
- Tag appropriately for discoverability
- Choose extension that matches content intent, not just topic

## Quality gate

After writing or editing markdown, run markdownlint on content/specs/skills.
See `.skills/markdownlint/SKILL.md`.

## Templates

Starter templates available in `assets/`:

### Content templates

- `template.service.md` - For architecture/system design posts
- `template.conf.md` - For principles/best practices posts
- `template.log.md` - For incident/story posts

### Shareable practice templates

- `template.adr.md` - Architecture Decision Record template
- `template.postmortem.md` - Incident postmortem template
- `template.coe.md` - Correction of Errors (COE) template

Copy and customize these templates when creating new content.
The practice templates are meant to be shared - they're
examples of our approach that others can use or improve.
