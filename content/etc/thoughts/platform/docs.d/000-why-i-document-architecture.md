---
title: "Why we document architecture (even when it feels like extra work)"
type: md
tags: [docs, architecture, platform, collaboration]
---

## Why this matters to me

Documentation isn't just about writing things down.

It's about helping everyone (including our future selves) understand and build on what we create together.

When we share what we learn, we make it easier for others to contribute, collaborate, and improve our systems.

## What I've noticed

Most teams see documentation as something extra — something to do after the "real work" is done.

But what if documentation **is** the real work?

When we document:

- We help others understand our thinking
- We make our systems more sustainable and maintainable
- We reduce resource waste (less time debugging, fewer repeated mistakes)
- We enable collaboration across teams and time zones

## What I mean by "documentation"

Documentation can take many forms:

- **Architecture Decision Records (ADRs)** — why we chose this path
- **Diagrams** — how components connect
- **Runbooks** — how to operate and troubleshoot
- **Code organization** — clear, modular structure
- **Configuration** — defaults that explain intent
- **Postmortems** — what we learned from incidents

Configuration is a form of documentation too — it's documentation that executes.

## Why I try to document even when nobody asks

### It helps us build more sustainably

When decisions are documented, we can:

- Build on top of them instead of starting over
- Understand why things exist before changing them
- Share knowledge across the team
- Make better use of our resources (time, energy, compute)

### It enables collaboration

Without documentation, only a few people can contribute safely.

With documentation, anyone can understand, improve, and extend our systems.

This is especially important in open source communities where collaboration is everything.

### It makes systems more accessible

When we document clearly:

- New team members can contribute faster
- AI agents can help more effectively
- Future maintainers can understand our intent
- Everyone saves time and energy

## This is a series

This post is the start of a series on documentation as engineering practice. Coming next:

- How ADRs help us make decisions together
- Postmortems as shared learning
- Configuration as living documentation
- Diagrams as shared understanding
- Documenting for AI collaboration
- When not to document

## Our approach (so far)

We've created some simple templates that work for us. They're stored in our repo and anyone
can use or improve them:

- **ADR template** - for architecture decisions
  - [`.skills/thinkos-content-creator/assets/template.adr.md`][adr-template]
- **Postmortem template** - for incident learning
  - [`.skills/thinkos-content-creator/assets/template.postmortem.md`][postmortem-template]
- **COE template** - for structured post-incident analysis
  - [`.skills/thinkos-content-creator/assets/template.coe.md`][coe-template]

These aren't perfect. They're just what we've found helpful. What would you change? What are we missing?

[adr-template]: https://github.com/guilherme-am/thinkctl/blob/main/.skills/thinkos-content-creator/assets/template.adr.md
[postmortem-template]: https://github.com/guilherme-am/thinkctl/blob/main/.skills/thinkos-content-creator/assets/template.postmortem.md
[coe-template]: https://github.com/guilherme-am/thinkctl/blob/main/.skills/thinkos-content-creator/assets/template.coe.md

## Let's explore this together

What's your experience with documentation? What works for your team? I'd love to learn from your approach.

## References

- [Google SRE Book: Table of Contents](https://sre.google/sre-book/table-of-contents/)
- [AWS Prescriptive Guidance: Building a Cloud Center of Excellence](https://docs.aws.amazon.com/prescriptive-guidance/latest/cloud-center-of-excellence/introduction.html)
- **Our templates:** [`.skills/thinkos-content-creator/assets/`](https://github.com/guilherme-am/thinkctl/tree/main/.skills/thinkos-content-creator/assets)
