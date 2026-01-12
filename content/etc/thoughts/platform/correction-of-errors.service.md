---
title: "Correction of Errors (COE): learning from incidents the AWS way"
type: service
tags: [platform, reliability, incidents, docs, operations]
---

## How we discovered COE (the hard way)

I've been in more war rooms than I care to count. The kind where you're on a bridge call at 11pm, watching production burn, customers complaining, and everyone scrambling to figure out what broke.

For years, we'd fix the issue, write a quick "what happened" doc, maybe add some TODOs to the backlog, and move on. Same types of incidents kept happening - different symptoms, same root causes we never actually fixed.

Then we started getting serious about it. Not because someone told us to, but because we were tired of repeating the same failures. We started doing postmortems. Then we found AWS's COE format and adapted it to actually track whether we were improving.

The results were real:
- Customer satisfaction went up
- Deployment time optimized significantly  
- We reduced incident response time by 45%
- Status reports actually became useful instead of vague hand-waving

But the real test came during a production database migration.

## The 2am migration that didn't fail

Monday night, 11pm. We're starting a production database migration for a telecom provisioning system - the kind that automatically provisions mobile and satellite network services. The upgrade required migrating PostgreSQL from version 9 to 14, with years of provisioning data and massive snapshots.

The plan was to finish by 5am Tuesday. Tight window. High stakes. Customers across multiple time zones depending on this system.

Here's the thing: we had a COE document from a similar deployment upgrade we'd done months earlier. Not the same system, but similar enough - large dataset, postgres upgrade, tight maintenance window.

Around 2am, we hit a snag in the migration steps. Data wasn't moving as expected. The team started troubleshooting, but something felt familiar. I pulled up that old COE document.

There it was - in the "Five Whys" section from the previous incident, we'd documented a specific bug in the migration tooling when dealing with large snapshot datasets. We'd even added an action item: "Document the workaround for future migrations."

Someone had actually done it. The workaround was right there in the action items section.

We applied it. Migration continued. We finished at 4:30am, ahead of schedule.

That's when COE clicked for me. It wasn't just about analyzing what went wrong - it was about building institutional memory that actually gets used.

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

## The hard parts nobody talks about

COE sounds great in theory. In practice, here's what actually happens:

### Getting people to write them

After a 12-hour incident, the last thing anyone wants to do is write a detailed analysis. I've seen COEs that took weeks to finish because everyone was "too busy." By then, memories are fuzzy and the urgency is gone.

**What worked for us:** Block time the day after the incident. Two hours, whole team, write it together while it's fresh. Yes, people are tired. Do it anyway.

### The Five Whys reveals organizational problems

Sometimes you ask "why" enough times and you end up at "because leadership decided to cut the infrastructure budget" or "because we don't have enough people to do this properly."

Now what? You can't put "hire more people" or "reverse executive decision" as an action item.

**What worked for us:** Document it anyway. Be honest. Sometimes the COE becomes evidence for why certain decisions need to be revisited. Sometimes it doesn't change anything, but at least the team knows their concerns are documented.

### Action items that never get done

This is the biggest one. You write a beautiful COE with 8 action items, assign owners, set deadlines. Three months later, 2 are done, 3 are "in progress," and 3 are forgotten.

**What worked for us:** 
- Review open COE action items in every sprint planning
- Limit action items to 3-5 max - if you have more, you're not prioritizing
- Track completion rate as a team metric (we aim for 80%+ within 30 days)
- If an action item keeps getting deprioritized, either make it smaller or admit it's not actually important

### Blameless is hard

Everyone says "blameless postmortem" but when someone fat-fingered a production command that took down the service, it's really hard not to focus on that person.

**What worked for us:** Always ask "why was this possible?" If someone can fat-finger a production command, that's a system design problem, not a people problem. The COE should focus on why we don't have safeguards, not why someone made a mistake.

### When the same thing breaks twice

This is the most demoralizing. You did a COE, you had action items, you thought you fixed it. Then it breaks again.

**What worked for us:** Reference the previous COE in the new one. Ask explicitly: "Why didn't our previous action items prevent this?" Sometimes you didn't go deep enough in the Five Whys. Sometimes the action items weren't actually implemented. Sometimes you fixed one path to failure but not all of them.

It's not failure - it's iteration.

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
