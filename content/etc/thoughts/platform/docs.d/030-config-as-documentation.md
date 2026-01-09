---
title: "Configuration as living documentation"
type: md
tags: [docs, config, automation, tooling, sustainability]
---

## Why configuration matters

Written documentation can drift from reality.
Configuration files can't — they're what the system actually runs.

When we configure our projects well, we create documentation that:

- Never goes stale
- Helps new contributors get started
- Reduces resource waste (less time debugging, fewer repeated questions)
- Makes our systems more sustainable and maintainable

## What configuration tells us

A repository's configuration reveals what the team values:

- `Cargo.toml` shows our dependencies and commitments
- Linting configuration shows what we consider important
- CI pipelines show what we actually enforce
- Directory structure shows how we organize our thinking

## Good defaults answer questions before they're asked

When defaults are clear:

- "How do I run this?" → `make run` or `cargo run`
- "How should I format this?" → formatter configuration + CI
- "Do we treat warnings as errors?" → CI configuration
- "How do we release?" → release scripts + CI

The best documentation is the kind we don't have to open.

## When configuration becomes a challenge

Configuration isn't magic either.

If we:

- Add tools without explaining why
- Enforce rules without documenting intent
- Create pipelines that only one person understands

...we've just moved the problem around.

## A practice that helps

**If a convention matters to us, we enforce it.**
**If we enforce it, we document why.**

This creates a feedback loop:

- Configuration enforces what matters
- Documentation explains the intent
- Contributors can understand and improve both

## Example: A sustainable approach

Here's how we might structure a project with clear defaults:

```toml
# Cargo.toml
[package]
name = "our-project"
edition = "2021"

[dependencies]
# We prefer minimal dependencies for sustainability
# Each dependency is a long-term maintenance commitment

[dev-dependencies]
# Tools that help us maintain quality
```

```yaml
# .github/workflows/ci.yml
# We run these checks to maintain our shared standards
# Everyone can see what matters to us
```

## Let's improve together

Most teams don't have a documentation problem.
They have a "unclear defaults" problem.

When we make our defaults clear and consistent, documentation naturally stays true.

What's your approach to configuration and defaults? I'd love to hear what works for your team.

## References

- [The Twelve-Factor App: Config](https://12factor.net/config)
- [AWS Prescriptive Guidance: Cloud Center of Excellence](https://docs.aws.amazon.com/prescriptive-guidance/latest/cloud-center-of-excellence/introduction.html)
