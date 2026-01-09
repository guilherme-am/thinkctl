---
title: "Building sustainable systems (what we believe)"
type: conf
tags: [philosophy, sustainability, clean-code, open-source, collaboration]
---

## What we mean by "sustainable"

Sustainability isn't just about carbon footprints or cloud costs (though those matter).

It's about building systems that:

- **Use resources wisely** - compute, memory, storage, energy, money, human time
- **Can be maintained long-term** - by us, by our teammates, by future contributors
- **Enable collaboration** - through clear code, good documentation, shared understanding
- **Compound over time** - each improvement makes the next one easier

We care about sustainability because technology should help people and our planet,
not drain them.

## What shaped our thinking

### Google SRE Book

The [Site Reliability Engineering book](https://sre.google/sre-book/table-of-contents/)
taught us that:

- Reliability is a feature, not an afterthought
- Toil should be automated away
- Systems need to be operable, not just functional
- Blameless postmortems help us learn together
- Monitoring tells us what's actually happening

We don't work at Google scale, but these principles work at any scale.

### Clean Code (Robert C. Martin)

Clean code isn't about perfection or dogma.

It's about:

- Making our intent clear to other humans (and our future selves)
- Reducing cognitive load
- Making changes safer and faster
- Enabling collaboration

Readable code is more sustainable than clever code.

### Linus and the Linux community

From Linus Torvalds and the Linux community, we learned:

- **Open source creates better software** - many eyes, many perspectives
- **Technical merit matters** - but so does collaboration
- **Code should speak for itself** - with good design and clear intent
- **Communities need governance** - but also trust and autonomy

The Linux kernel proves that sustainable, collaborative development can build
systems that power the world.

### Open source values

We believe in:

- **Sharing what we learn** - documentation, code, templates, mistakes
- **Contributing back** - to projects we use and communities we're part of
- **Building in the open** - transparency helps everyone
- **Learning from others** - we don't have all the answers

Open source isn't just about license - it's about how we work together.

## Our principles

### 1. Resource efficiency matters

Every resource has a cost:

- **Compute** - energy, money, environmental impact
- **Storage** - energy, money, maintenance burden
- **Network** - latency, cost, carbon
- **Human time** - the most valuable and least renewable

We optimize for all of these, not just one.

### 2. Clean code is sustainable code

Code that's easy to read is:

- Easier to debug
- Safer to change
- Faster to onboard into
- More welcoming to contributors

We prefer clarity over cleverness.

### 3. Modularity enables reuse

Well-organized code:

- Can be reused instead of rewritten
- Can be understood in pieces
- Can be tested independently
- Can be replaced without breaking everything

Good boundaries reduce waste.

### 4. Automation reduces toil (when done thoughtfully)

We automate:

- Repetitive tasks that waste human time
- Things that need to happen consistently
- Checks that prevent common mistakes

We don't automate:

- Things that need human judgment
- Things that change frequently
- Things that would be harder to understand automated

Automation should reduce toil, not hide complexity.

### 5. Collaboration compounds

When we:

- Document our decisions
- Share our learnings
- Write clear code
- Create reusable patterns
- Help others contribute

...we make it easier for everyone to build better systems together.

Sustainability is a collective effort.

## How this shows up in practice

### In our code

- Clear naming that explains intent
- Modular design that enables reuse
- Tests that document behavior
- Comments that explain "why," not "what"

### In our infrastructure

- Right-sized resources (not over-provisioned)
- Efficient data storage (not keeping everything forever)
- Automated scaling (using resources when needed)
- Observability that helps us understand actual usage

### In our processes

- Documentation that helps others contribute
- Blameless postmortems that help us learn
- Architecture decisions that explain trade-offs
- Code reviews that share knowledge

### In our community

- Open source contributions
- Sharing templates and patterns
- Helping others learn
- Learning from others' experiences

## AI and agents fit here too

AI agents can help us build more sustainably:

- Automating repetitive coding tasks
- Helping with documentation
- Suggesting optimizations
- Catching common mistakes

But only if we use them thoughtfully:

- Humans review important decisions
- We document what agents do
- We validate their suggestions
- We keep humans in the loop for critical systems

AI is a tool for collaboration, not a replacement for human judgment.

## This is a living philosophy

These beliefs will evolve. What we learn next year will shape what we believe.

We're not claiming we do this perfectly. We're claiming we care about it and
we're trying to improve.

What do you think? What are we missing? What would you add?

## References

- [Google SRE Book](https://sre.google/sre-book/table-of-contents/)
- [Clean Code by Robert C. Martin](https://www.oreilly.com/library/view/clean-code-a/9780136083238/)
- [Just for Fun: The Story of an Accidental Revolutionary by Linus Torvalds](https://www.goodreads.com/book/show/160171.Just_for_Fun)
- [The Cathedral and the Bazaar by Eric S. Raymond](http://www.catb.org/~esr/writings/cathedral-bazaar/)
- [Open Source Initiative](https://opensource.org/)
