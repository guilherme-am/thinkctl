---
title: "When not to document"
type: md
tags: [docs, pragmatism, sustainability]
---

## An honest perspective

Not everything needs documentation.

Sometimes documentation is:

- A way to avoid fixing the real problem
- Busywork that looks productive
- A second source of truth that quickly becomes stale

Let's talk about when **not** to document.

## Don't document what the system can prove

If the answer is in code, tests, and CI, we don't need a second source of truth.

Examples:

- **Formatting rules** → formatter configuration + CI
- **API contracts** → schema + tests
- **Build steps** → build scripts + CI

If the system can validate it, let it.

This is more sustainable — we maintain one source of truth instead of two.

## Don't document things that change frequently

If something changes weekly, written prose will be wrong within days.

Instead, we can:

- Document the stable pattern or principle
- Document how to discover the current state

"Run this command to see what's deployed" is better than "Here's what's deployed (as of last Tuesday)."

## Don't document opinions without context

"We always do X" is rarely true for long.

Better to write:

- "We do X because of constraint Y"
- "We prefer X over Z because of our resource/cost/team context"

This helps future us understand when circumstances have changed enough to reconsider.

## Do document the sharp edges

The most valuable documentation helps us avoid painful mistakes:

- The failure mode that's not obvious
- The migration path that looks safe but isn't
- The configuration that will wake someone up at 3am

These are worth documenting clearly.

## Documentation as tool, not virtue

Documentation isn't inherently good.
It's a tool that helps when used appropriately.

Let's use it where it:

- Reduces risk
- Reduces resource waste
- Enables collaboration
- Makes knowledge accessible

And let's skip it when it:

- Duplicates what the system can prove
- Will be stale immediately
- Substitutes for fixing the underlying problem

## Let's learn together

How does your team decide what to document? What's your approach to keeping documentation sustainable?

I'm always learning from how others handle this balance.

## References

- [Documentation-Driven Development](https://gist.github.com/zsup/9434452)
- [Write the Docs: Minimalism in Documentation](https://www.writethedocs.org/)
