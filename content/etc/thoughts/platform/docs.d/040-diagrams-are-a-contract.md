---
title: "Diagrams as shared understanding"
type: md
tags: [docs, diagrams, architecture, collaboration]
---

## Why we draw diagrams

We draw diagrams not because they're pretty, but because they help us build shared understanding.

Words can be ambiguous.
Two people can "agree" in a meeting and then build two different things.

Diagrams help us see if we're really aligned.

## What good diagrams do for us

When we create clear diagrams:

- Boundaries become visible
- Dependencies become explicit
- Failure modes become discussable
- Context becomes shareable (useful for reviews and incidents)

It's not about being perfect.
It's about being clear enough to reason together.

## Diagram types that help us most

### Context diagram

Shows what talks to what, at a high level.

Helps new team members (and our future selves) understand the big picture.

### Sequence diagram

Shows one request, end-to-end.

Even better: show the happy path AND what happens when something fails.

### State diagram

If our system has retries, queues, or workflows, it has state.

Drawing it helps us see where things might get stuck or lose data.

## The challenge: keeping diagrams current

If diagrams drift from reality, they become misleading rather than helpful.

Some practices that help:

- Keep diagrams close to the code (in the repo)
- Keep them small and focused
- Update them as part of changes, not as separate work
- Use tools that can generate diagrams from code when possible

## Diagrams as collaboration tools

Diagrams don't solve architecture.
They solve communication.

And since architecture is mostly about helping people build together under constraints,
good diagrams make our work more sustainable and collaborative.

## Let's learn together

What diagram types work best for your team? How do you keep them current? I'm always curious about different approaches.

## References

- [The C4 model for visualizing software architecture](https://c4model.com/)
- [Simon Brown's Software Architecture for Developers](https://simonbrown.je/)
- [Mermaid - Diagramming tool](https://mermaid.js.org/)
