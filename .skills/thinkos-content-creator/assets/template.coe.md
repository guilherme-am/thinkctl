---
title: "Correction of Errors (COE) template"
type: md
tags: [coe, incidents, reliability]
---

## Correction of Errors (COE)

**Date:** YYYY-MM-DD  
**Incident window:** [start] to [end] (UTC)  
**Severity:** [P0/P1/P2/P3]  
**Owner:** @name  
**Reviewers:** @names  
**Status:** [Draft/In Review/Complete]

## Summary (write this last)

Write this as if it will be forwarded to stakeholders (CEO, customers, etc).
One paragraph that stands alone. Include:
- What broke
- Customer impact (who, how many, how long)
- Root cause (one sentence)
- What we did to fix it
- What we're doing to prevent recurrence

## Impact

### Customer Impact
- **Affected users:** [number or percentage]
- **Duration:** [total time from first impact to full recovery]
- **User experience:** [what did they see/experience?]
- **Business impact:** [revenue loss, SLA breach, reputation, etc]

### System Impact
- **Services affected:** [list]
- **Error rate:** [percentage or count]
- **Latency impact:** [if applicable]
- **Data integrity:** [any data loss or corruption?]

## Timeline (UTC)

Write this while the incident is fresh. Include:
- When things started breaking (not when you noticed)
- When you got alerted
- Key investigation steps
- What you tried that didn't work
- What actually fixed it
- When you confirmed recovery

**Template:**
- `HH:MM` — [Event description with context]
- `HH:MM` — [Action taken by whom]
- `HH:MM` — [Observation or discovery]

**Example:**
- `23:15` — Deployment of v2.4.1 to production started
- `23:22` — First customer reports of 500 errors via support
- `23:25` — On-call engineer paged via PagerDuty
- `23:30` — Identified spike in database connection errors
- `23:35` — Attempted connection pool restart (did not resolve)
- `23:45` — Rolled back deployment to v2.4.0
- `23:52` — Error rate returned to normal
- `00:05` — Confirmed with monitoring that all systems nominal

## Metrics

What should have alerted us? What did alert us? What didn't we have?

### Metrics that detected the issue:
- [metric name] - [dashboard/alarm link]

### Metrics that should have detected it earlier:
- [what we're missing]

### Key metrics during incident:
- Error rate: [before/during/after]
- Latency: [p50/p95/p99]
- Traffic: [requests/sec]

## Incident questions

### Detection
- **When did we learn there was customer impact?** [timestamp]
- **How did we learn?** [customer report / monitoring / alarm / other]
- **Time from impact to detection:** [duration]
- **How could we reduce time-to-detect by half?** [specific improvement]

### Diagnosis
- **What was the underlying cause?** [technical root cause]
- **Time from detection to diagnosis:** [duration]
- **What slowed down diagnosis?** [missing logs, unclear metrics, etc]
- **Was there any internal activity during the incident?** [deploy, maintenance, config change]
- **How could we reduce time-to-diagnose by half?** [specific improvement]

### Mitigation
- **When did customer impact return to normal?** [timestamp]
- **How did we verify recovery?** [metrics, customer reports, testing]
- **Time from diagnosis to mitigation:** [duration]
- **What did we try that didn't work?** [important to document]
- **How could we reduce time-to-mitigate by half?** [specific improvement]

## 5 (or more) Whys

Keep asking "why" until you get to systemic issues, not just symptoms.
If you end at "human error," keep going - ask why the error was possible.

**The problem:** [state the problem clearly]

1. **Why did [problem] happen?**
   - Because [immediate cause]

2. **Why did [immediate cause] happen?**
   - Because [deeper cause]

3. **Why did [deeper cause] happen?**
   - Because [even deeper cause]

4. **Why did [even deeper cause] happen?**
   - Because [systemic issue]

5. **Why did [systemic issue] exist?**
   - Because [root cause - usually a gap in process, tooling, or design]

**Example:**
1. Why did the service crash?
   - Because it ran out of memory
2. Why did it run out of memory?
   - Because of a memory leak in the new code
3. Why didn't we catch the memory leak before production?
   - Because our staging environment doesn't run long enough to trigger it
4. Why doesn't staging run long enough?
   - Because we tear it down nightly to save costs
5. Why don't we have long-running test environments?
   - Because we don't have a process for sustained load testing

## Action items

**Rules:**
- Limit to 3-5 items max (if you have more, you're not prioritizing)
- Each must have owner and due date
- Try to include at least one item for prevention, detection, and mitigation
- Be specific: not "improve monitoring" but "add alarm for X metric when Y threshold"
- Track completion in sprint planning

**Template:**
- [ ] **[Prevention/Detection/Mitigation]** [Specific action] — @owner — YYYY-MM-DD

**Example:**
- [ ] **Prevention:** Add memory limits to service container config — @alice — 2026-01-20
- [ ] **Detection:** Create alarm for memory usage >80% sustained for 5min — @bob — 2026-01-18
- [ ] **Mitigation:** Document rollback procedure in runbook — @charlie — 2026-01-15
- [ ] **Process:** Implement 48hr sustained load test in staging — @alice — 2026-02-01

## References

Link everything. Future you will thank present you.

- **Incident chat:** [Slack thread]
- **Monitoring:** [Grafana dashboard, CloudWatch, etc]
- **Alerts:** [PagerDuty incident, alarm history]
- **Logs/Traces:** [CloudWatch Logs Insights query, X-Ray trace]
- **Related PRs:** [deployment that caused it, fix PR]
- **Related COEs:** [if this is a repeat or similar incident]
- **Customer tickets:** [support ticket numbers]

---

## Tips for writing good COEs

**Do it while it's fresh:** Block 2 hours the day after the incident. Write it as a team.

**Be honest:** If leadership decisions contributed, document it. If someone made a mistake, focus on why it was possible, not who did it.

**Make it searchable:** Use specific error messages, service names, and technical terms. Future you searching for "postgres migration snapshot" should find this.

**Update it:** If action items don't get done, update the COE with why. If the same thing breaks again, reference this COE in the new one.

**Review old COEs:** Every quarter, look at COEs from 3-6 months ago. Did we actually do the action items? Did they work?
