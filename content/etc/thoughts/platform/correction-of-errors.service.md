---
title: "Correction of Errors (COE): learning from incidents the AWS way"
type: service
tags: [platform, reliability, incidents, docs, operations]
---

## How I discovered COE

A few months ago, I was reading through AWS's operational excellence documentation (as one does at 2am when you can't sleep), and I stumbled across something called "Correction of Errors" or COE.

At first, I thought it was just another name for postmortems. But the more I read, the more I realized it was something a bit different — not necessarily better, just... more structured in a way that resonated with how I think about incidents.

## The problem with postmortems (sometimes)

Don't get me wrong — I love a good postmortem. When a team sits down after an incident and honestly talks through what happened, what went wrong, and what they learned, that's incredibly valuable.

But I've also seen postmortems that are... less useful. You know the ones:
- Written in a rush because everyone just wants to move on
- Full of vague action items like "improve monitoring" with no owner or deadline
- Stored in some wiki that nobody ever looks at again
- More focused on blame avoidance than actual learning

And sometimes, even when the postmortem is good, the action items just... disappear. They get added to the backlog, deprioritized, forgotten.

## What makes COE different

COE isn't radically different from a postmortem. It's more like a postmortem with a specific structure that forces you to think about certain things.

The AWS COE format walks through these sections:

**1. Summary** (written last, after you understand everything)

**2. Impact** — Who was affected? For how long? What did they experience?

**3. Timeline** — What happened, when, and who did what. This is the story of the incident.

**4. Metrics** — Actual numbers. Error rates, latency, affected requests, downtime duration. Quantify the impact.

**5. Incident Questions** — This is where it gets interesting:
- **Detection**: How did we find out? Could we have known sooner?
- **Diagnosis**: How long did it take to figure out what was wrong? What slowed us down?
- **Mitigation**: What actually fixed it? What did we try that didn't work?

**6. Five Whys** (or more) — Keep asking "why" until you get past the immediate cause to the systemic issue.

**7. Action Items** — Concrete improvements with owners and deadlines. Not "improve monitoring" but "Add alerting for X metric, owner: Alice, due: next sprint."

## Why I like this structure

The "Incident Questions" section is what really clicked for me. It forces you to think about the incident in three phases:

- Could we have **detected** this earlier?
- Could we have **diagnosed** it faster?
- Could we have **mitigated** it more gracefully?

And the Five Whys helps you avoid the trap of stopping at the surface-level cause. "The service crashed" → "Why?" → "Out of memory" → "Why?" → "Memory leak" → "Why?" → "No resource limits" → "Why?" → "We don't have a standard for setting resource limits" → Ah, there's the systemic issue.

## When I use COE vs. postmortems

Honestly? It depends on the team and the situation.

If the team already has a postmortem culture they're comfortable with, I'm not going to force COE on them. The format matters less than the habit of learning from incidents.

But I've found COE useful when:
- The incident was significant and we need to track improvements across multiple teams
- There's a risk of action items getting lost
- We want to be really systematic about prevention, detection, and mitigation
- The team is new to incident analysis and needs more structure

For smaller incidents, a lightweight postmortem is totally fine. COE is more heavyweight, and that's okay — not every incident needs the full treatment.

## My COE template

I put together a template based on the AWS format, adapted for how I actually use it:

`.skills/thinkos-content-creator/assets/template.coe.md`

It's not perfect. I'm still iterating on it. If you use something similar, I'd love to hear what works for you.

## The real goal

Whether you use COE, postmortems, or something else entirely, the goal is the same: **learn from incidents and actually improve**.

The format is just a tool. What matters is:
- Being honest about what happened
- Understanding why it happened
- Doing something about it
- Following through on the action items

If your current postmortem process does that, great! Keep doing it. If you're looking for more structure, maybe COE is worth trying.

## References

- [AWS: Creating a correction of errors document](https://aws.amazon.com/pt/blogs/mt/creating-a-correction-of-errors-document/)

---

Have you used COE or something similar? What's your approach to learning from incidents? I'm always curious to hear how other teams handle this.
