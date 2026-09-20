---
title: "whoami"
type: conf
---

## whoami

```bash
$ whoami
guilherme

$ cat /etc/passwd | grep guilherme
guilherme:x:1000:1000:Platform Engineer:/home/guilherme:/bin/bash
```

Hi! I'm Guilherme.

I build systems that help people (and machines) work together, the infrastructure kind, mostly. Platform engineering, reliability,
observability. The stuff that makes other things possible.

## What I Do

I've worked across different domains:

- **Telecom** - 5G/6G networks, edge computing, distributed systems
- **Automotive R&D** - Real-time infrastructure for autonomous systems
- **SaaS Platforms** - Cloud-native apps, DevOps, SRE practices
- **Platform / DevEx** - Kubernetes, IaC, GitOps, the paved road
- **Agents** - Productionizing agent systems, not only demos

The last one is the main work now. I still come from infra-as-code and developer
experience. The question I keep asking is: what does a self-running agent stack
look like if we treat it like a real system?

Definition (DeepAgents, Google ADK, Jev). Tooling (LangChain, Claude SDK, OpenAI
SDK, A2A). Least-privilege access. Monitoring the platform *and* the agent
traces. Auto-evals. Refinement datasets so we can fine-tune instead of hoping
the next prompt is luckier.

Right now, I'm exploring how AI agents can help us build better systems, not
replace us, but work alongside us, if we operate them with the same discipline
we already use for platforms.

## What I Believe

I think **systems exist to connect people**, not just machines.

Reliability isn't just about uptime. It's about trust, predictability, and keeping promises to the people who depend
on your systems.

Automation should reduce toil, but it shouldn't hide complexity. We need to understand what we're building.

And sustainability matters. Not just environmental (though that's important), but building systems that don't burn
people out, that use resources wisely, that can be maintained long-term.

## What I Love

I love **open source**. The community, the collaboration, the spirit of building things together and sharing what we learn.

I love **Linux and systems**. There's something beautiful about understanding how things work at a fundamental level.

I love **learning from others**. Every conversation, every code review, every incident postmortem teaches me something new.

## What I'm Learning

I'm always learning. Right now:

- How to productionize agents with evals, traces, and least privilege
- Better ways to build platform infrastructure
- How to write about technical things in a way that's actually helpful
- How to balance sustainability with performance

I don't have all the answers. I'm figuring it out as I go, just like everyone else.

## Get in Touch

```bash
$ cat /etc/thinkctl/contact.conf
github: guilherme-am
email: guilherme.amoreira96@gmail.com
linkedin: /in/guilherme-amoreira
```

If you want to chat about systems, open source, sustainability, or anything else, reach out! I'd love to hear from you.

## System Status

```bash
$ systemctl status guilherme.service

● guilherme.service - Platform Engineering & Systems Thinking
   Loaded: loaded (/etc/systemd/system/guilherme.service)
   Active: active (running)
   
   ├─ platform-engineering.service     active
   ├─ open-source.service              active
   ├─ learning.service                 active
   ├─ sustainability.target            active
   └─ ai-agents.service                active
```

---

Thanks for stopping by. Let's build something good together.
