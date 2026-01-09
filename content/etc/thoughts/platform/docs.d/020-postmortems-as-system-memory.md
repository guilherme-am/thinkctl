---
title: "Postmortems as shared learning"
type: md
tags: [docs, reliability, incidents, sre, collaboration]
---

## Why we write postmortems

We write postmortems not to assign blame, but to learn together.

When something goes wrong (and it will), we want to:

- Understand what happened
- Share what we learned
- Help others avoid similar issues
- Build more resilient systems together

## Incidents are opportunities to learn

Most incidents aren't unique.
They're patterns we can learn from and share.

The difference between teams that improve and teams that repeat the same mistakes
is whether they capture and share what they learn.

Postmortems are that shared memory.

## What we try to capture

### Timeline

When did things happen? This helps us understand cause and effect.

### Impact

Who was affected? How? For how long?

We also try to capture the human impact: interrupted sleep, stress, context switching costs.

### Contributing factors

Rarely is there a single cause.
Usually, multiple things aligned:

- Monitoring didn't alert (or alerted too late)
- Documentation was unclear
- The deploy was riskier than we thought
- Knowledge was concentrated in one person
- Rollback was more complex than expected

### What we changed

A postmortem without follow-up actions is just a story.

What can we do to make this less likely next time? Or to recover faster?

## Blameless means learning-focused

"Blameless" doesn't mean "no one was involved."

It means:

- We focus on improving the system, not punishing people
- We write down the conditions that made the failure likely
- We invite everyone to contribute to the solution

When postmortems feel like punishment, people stop sharing. And silence is expensive.

## Our postmortem template (designed for 3am)

When we're tired and stressed, simple structures help. This is what works for us.

It might be too simple, or maybe it's missing something important for your context. What would you add?

```markdown
# Incident: [Short descriptive title]

**Date:** YYYY-MM-DD
**Duration:** [HH:MM] to [HH:MM] ([X hours/minutes])
**Severity:** [Low | Medium | High | Critical]
**Author:** @name

## Summary

One paragraph: what happened, who was affected, how we resolved it.

## Impact

- **Users affected:** [number/percentage/scope]
- **Services affected:** [which systems]
- **Duration:** [total time]
- **Business impact:** [if any - revenue, SLA, reputation]
- **Team impact:** [people woken up, hours spent, stress level]

## Timeline (all times UTC)

- **HH:MM** - First symptom noticed (by whom, how)
- **HH:MM** - Investigation started
- **HH:MM** - [key event]
- **HH:MM** - Root cause identified
- **HH:MM** - Fix applied
- **HH:MM** - Verified resolved
- **HH:MM** - Incident closed

## What we learned

### Contributing factors

Not looking for a single "root cause" - usually multiple things aligned:

1. [Factor - e.g., monitoring didn't alert]
2. [Factor - e.g., documentation was unclear]
3. [Factor - e.g., deploy timing]

### What worked well

Things that helped us recover:

- [thing that helped]
- [thing that helped]

### What we could improve

Areas where we struggled:

- [challenge]
- [challenge]

## Actions

### Immediate (already done)

- [x] [action taken to resolve] - @owner

### Short-term (next 2 weeks)

- [ ] [action to prevent recurrence] - @owner - [due date]
- [ ] [action to detect faster] - @owner - [due date]

### Long-term (next quarter)

- [ ] [structural improvement] - @owner - [due date]

## References

- **Alert:** [link]
- **Dashboard:** [link]
- **Chat discussion:** [link]
- **PR/fix:** [link]

---

**This is blameless.** We're learning together, not looking for someone to blame.
If you have ideas for how we could improve this postmortem format or our
response process, please share.
```

You can find this template in our repo at:
[`.skills/thinkos-content-creator/assets/template.postmortem.md`](https://github.com/guilherme-am/thinkctl/blob/main/.skills/thinkos-content-creator/assets/template.postmortem.md)

## Let's learn together

The goal isn't to never fail.
The goal is to create systems that can recover, and to share what we learn so everyone benefits.

What's your team's approach to postmortems? I'd love to learn from your experience.

## Where this fits (COE / post-incident analysis)

If you’re thinking about “where do postmortems live?”, for us they live in our post-incident
analysis practice.

AWS has a concrete approach called **Correction of Errors (COE)**. We wrote down the parts
we like (and our own template) here:

- `/etc/thoughts/platform/correction-of-errors`

## References

- [Google SRE Book: Postmortem Culture](https://sre.google/sre-book/postmortem-culture/)
- [Google SRE Book: Example Postmortem](https://sre.google/sre-book/postmortem/)
- [Blameless Postmortems by John Allspaw](https://www.etsy.com/codeascraft/blameless-postmortems/)
