---
title: "Documenting for AI collaboration"
type: md
tags: [docs, agents, ai, platform, collaboration]
---

## How documentation is changing

We're increasingly working with AI agents as collaborative partners.

This doesn't mean replacing people — it means finding new ways to work together, combining human creativity with AI capabilities.

When we document well, we help both humans and AI agents contribute effectively.

## What AI agents need to collaborate well

Humans can often "fill in the gaps" with context and intuition.
AI agents can't do this as well — they need clearer guidance.

So the documentation that helps AI agents the most is also the documentation that helps humans under pressure:

- **Clear file locations** — where things are
- **Explicit commands** — how to run, test, deploy
- **Explicit constraints** — what matters (cost, latency, security)
- **Ownership boundaries** — who owns what, how to contribute

## Three layers that help

### 1. Predictable structure

When our projects follow consistent patterns:

- Everyone navigates faster (humans, scripts, agents)
- We reduce cognitive load
- We make our systems more sustainable and maintainable

### 2. Executable documentation

Configuration, linters, formatters, CI pipelines — these are documentation that runs.

They help agents (and humans) understand what "good" looks like by showing it.

### 3. Decision records

AI agents are good at applying patterns.
They're not good at reading our team's mind.

When we write down:

- Why we rejected an approach
- What constraints matter for our context
- What success looks like for our system

...we help both agents and humans make better decisions.

## AI as collaborative partner

AI isn't a rival.
It's a tool that can help us work faster and more effectively — if we guide it well.

For changes that affect security, cost, or reliability, humans should always review and approve.

We're building systems where humans and AI complement each other's strengths.

## Keeping documentation sustainable

The goal isn't to write for machines instead of humans.

The goal is to write in a way that's clear, honest, and helpful for everyone — whether they're human contributors,
future maintainers, or AI agents helping with repetitive tasks.

Good documentation reduces ambiguity for all of us.

## Let's explore this together

How is your team working with AI agents? What documentation practices help? What challenges have you found?

I'm still learning and would love to hear your experience.

## References

- [Agent Skills overview](https://agentskills.io/home)
- [Kiro CLI steering docs](https://kiro.dev/docs/cli/steering/)
- [AGENTS.md standard](https://agents.md/)
- [Anthropic: Claude Agent SDK overview](https://platform.claude.com/docs/en/agent-sdk/overview)
- [Claude Code setup](https://code.claude.com/docs/en/setup)
- [Gemini CLI (GitHub)](https://github.com/google-gemini/gemini-cli)
- [Gemini CLI documentation](https://geminicli.com/docs/)
- [OpenAI Codex CLI](https://developers.openai.com/codex/cli/)
- [OpenAI Codex (GitHub)](https://github.com/openai/codex)
- [OpenAI: GPT Best Practices](https://platform.openai.com/docs/guides/gpt-best-practices)
- [Anthropic: Claude Prompt Engineering](https://docs.anthropic.com/claude/docs/introduction-to-prompt-design)

In this repo:

- `AGENTS.md` (workspace steering for agent tools; see “After writing or editing markdown, run markdownlint”)
- `.cursor/rules/skills-discovery.mdc` (ensure skills discovery under `.skills/`)
- `.kiro-cli/rules/rust-rules.mdc` (Kiro CLI project rules mirror)
- `.skills/markdownlint/SKILL.md` (how we run markdownlint)
