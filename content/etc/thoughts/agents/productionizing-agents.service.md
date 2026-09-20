---
title: "Productionizing agents"
type: service
tags: [agents, observability, evals, platform, security]
date: 2026-06-18
---

## The demo is not the system

By June I had already spent enough nights watching an agent look brilliant in a
notebook and then fall over as soon as it had real tools, real credentials, and
a real user waiting.

I come from infra as code and developer experience. Kubernetes, GitOps, least
privilege, dashboards that still make sense at 3am. That background started
feeling less like a previous chapter and more like the missing half of the
agent conversation.

A lot of agent write-ups stop at "the model called a tool." Production starts
after that. Who is allowed to call the tool. What we log. How we know the
answer was any good. What we do with the failures besides screenshot them.

## The loop I keep drawing

I have been treating the stack as one path, not a pile of SDKs:

```text
definition -> tools -> least privilege -> traces -> evals -> datasets
```

Definition is where the agent is declared. DeepAgents, Google's agent SDK, Jev
for bounded decisions, sometimes a thinner wrapper around Claude or OpenAI.
The important part is that the contract is written down: what it can do, what
it must not do, what "done" looks like.

Tools are the messy middle. LangChain, A2A, the vendor SDKs. This is where
people hide a filesystem and a production database behind "the agent will
figure it out." I do not want it to figure out access. I want the same
least-privilege story we already use for workloads: scoped credentials, short
lived tokens, no god-mode service account because the demo was easier that way.

Then observability. Two layers, on purpose. The platform still needs
Prometheus, Grafana, OpenTelemetry, the usual "is the thing up" questions. The
agent stack needs traces and evals: Langfuse, LangSmith, whatever lets you see
the chain of thought as an operation, not as a chat transcript you scroll
through once.

Evals are how I stop arguing with vibes. Auto-eval the cheap cases. Sample the
expensive ones. Keep humans on the ones that spend money or page someone.

The last step is the one I care about most lately: the failures become a
dataset. Not a graveyard of screenshots. A refinement set you can train or
fine-tune against later, so reliability is a loop instead of a prompt edit.

## Why the infra muscle still matters

If you have run a platform, this shape is familiar. Declare the unit. Give it
the smallest permissions that work. Watch it. Measure it. Feed the misses back
into the next version.

Agents add semantic uncertainty on top of the usual operational uncertainty.
That is why I have been paying attention to decision primitives as well as
frameworks. A generate-JSON-and-parse loop is a lot of tokens pretending to be
a gate. Some of those gates should be typed questions. Some of them should
still be an LLM writing prose. Mixing them without noticing is how you get a
system that cannot be operated.

I am not dropping IaC or DevEx. I am pointing that muscle at a new runtime.

## What I am still unsure about

I do not have a clean reference architecture I would stamp on every team. The
SDKs move. Eval quality is uneven. Fine-tuning from agent traces is easy to
talk about and easy to do badly.

The direction is clear enough to write down: treat agents as production
systems. Definition, tools, privilege, traces, evals, datasets. If a piece of
that loop is missing, I do not think you have an autonomous system yet. You
have a demo with extra steps.

If you are walking the same path, I would love to compare notes. What did you
keep from platform engineering, and what did you have to invent?

## Related units

- [Typed decision boundaries](/etc/thoughts/agents/typed-decision-boundaries)
- [Documenting for AI collaboration](/etc/thoughts/agents/documenting-for-ai-collaboration)
- [Building sustainable systems](/etc/thoughts/philosophy/sustainable-systems)
