---
title: "ADRs help us make decisions together"
type: md
tags: [docs, adr, architecture, collaboration]
---

## What an ADR can do for us

An Architecture Decision Record (ADR) helps us:

- Remember why we chose this approach
- Share context with new team members
- Revisit decisions when circumstances change
- Collaborate more effectively across teams

It's not bureaucracy — it's shared memory.

## What we capture in an ADR

### The context

What problem are we solving? What constraints do we have?

### The decision

What did we decide to do?

### The alternatives we considered

What other approaches did we explore? Why didn't we choose them?

### The consequences

What are the trade-offs? What does this decision unlock? What does it make harder?

### What might change our mind later

Circumstances change. What would make us revisit this decision?

## Why this helps

When we write down our decisions:

- We can build on them instead of re-debating them
- We help others understand our thinking
- We make it easier to change course when needed
- We create a more sustainable, collaborative environment

## Our ADR template (feel free to adapt it)

This is the template we use. It's simple and focused on helping us remember context and trade-offs.

Is it perfect? Probably not. What would you improve?

```markdown
# [Decision Title]

**Date:** YYYY-MM-DD
**Status:** Proposed | Accepted | Superseded
**Deciders:** @names of people involved

## Context

What problem are we trying to solve?

What constraints matter? (cost, time, team skills, existing systems)

## Decision

We will [do this thing].

## Alternatives we considered

### Option A: [alternative]

Why we didn't choose it:
- [reason]

### Option B: [alternative]

Why we didn't choose it:
- [reason]

## Consequences

### What this enables

- [positive outcome]
- [positive outcome]

### Trade-offs we're accepting

- [what we're giving up]
- [what becomes harder]

### What might change our mind

Circumstances that would make us revisit this:
- [condition]
- [condition]

## References

- [link to relevant discussion]
- [link to relevant documentation]
- [link to relevant code]

---

**Questions? Ideas for improvement?**
This is how we're approaching decisions right now, but we're always learning.
What would you change?
```

You can find this template in our repo at:
[`.skills/thinkos-content-creator/assets/template.adr.md`](https://github.com/guilherme-am/thinkctl/blob/main/.skills/thinkos-content-creator/assets/template.adr.md)

## Let's collaborate

Do you use ADRs in your projects? What format works for your team? I'm always learning from how others approach this.

## References

- [Documenting Architecture Decisions by Michael Nygard](https://cognitect.com/blog/2011/11/15/documenting-architecture-decisions)
- [ADR GitHub organization](https://adr.github.io/)
