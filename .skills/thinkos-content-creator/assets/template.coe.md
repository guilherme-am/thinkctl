---
title: "Correction of Errors (COE) template"
type: md
tags: [coe, incidents, reliability]
---

## Correction of Errors (COE)

**Date:** YYYY-MM-DD  
**Incident window:** [start] to [end] (UTC)  
**Owner:** @name  
**Reviewers:** @names  

## Summary (write this last)

Write this as if it will be forwarded to stakeholders.
One paragraph that stands alone.

## Impact

What happened to users/customers? For how long? What was the measurable impact?

## Timeline (UTC)

- HH:MM — event
- HH:MM — event

## Metrics

What metrics should have shown this? What metrics did show it?

- dashboards:
- alarms:
- logs/traces:

## Incident questions

### Detection

- When did we learn there was customer impact?
- How did we learn?
- How could we reduce time-to-detect?

### Diagnosis

- What was the underlying cause of customer impact?
- Was there any internal activity (deploy, maintenance) during the incident?
- How could we reduce time-to-diagnose?

### Mitigation

- When did customer impact return to normal?
- How did we verify recovery?
- How could we reduce time-to-mitigate?

## 5 (or more) Whys

- Why?
  - Because…
- Why?
  - Because…

## Action items

Each action item should have an owner and due date.
We try to include prevention, detection, and mitigation improvements.

- [ ] Action item — @owner — YYYY-MM-DD
- [ ] Action item — @owner — YYYY-MM-DD

## References

- links to alerts, dashboards, traces, PRs, chat logs

---

This is our current COE template. It’s not perfect.
What do you think is missing?
