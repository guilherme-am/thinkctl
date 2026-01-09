---
title: "Correction of Errors (COE): post-incident analysis as a reusable system"
type: service
tags: [platform, reliability, incidents, docs, operations]
---

## Why this exists

We like postmortems because they help us learn.

COE is a more structured way to do that learning and turn it into action items that prevent
reoccurrence, improve detection/diagnosis, and make operations more sustainable.

## The AWS COE shape (what we’re borrowing)

AWS walks through a COE document section-by-section:

- Summary (written last)
- Impact
- Timeline
- Metrics
- Incident Questions (detection/diagnosis/mitigation)
- 5 (or more) Whys
- Action Items

## Our COE template (imperfect, shareable)

We keep ours in the repo so we can evolve it with feedback:

- `.skills/thinkos-content-creator/assets/template.coe.md`

If you use COE in your org, what would you add or remove?

## References

- [AWS: Creating a correction of errors document](https://aws.amazon.com/pt/blogs/mt/creating-a-correction-of-errors-document/)
