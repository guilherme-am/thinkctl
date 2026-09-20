---
title: "Typed decision boundaries"
type: conf
tags: [agents, python, pydantic, llm]
date: 2026-09-20
---

## The pydantic memory

I still remember when Pydantic stopped being "that validation library" and
became something you expected to see at the edge of a Python service.

Before that, a lot of us treated incoming data as a dict we would poke at until
it looked safe. A missing key here, a string that was supposed to be an int
there, a handwritten parser that only the original author trusted. The program
had a shape in our heads. The data did not.

Pydantic made that boundary explicit. Untrusted input goes in. A typed object
comes out, or you get an error you can handle. The application starts after the
contract holds.

I have been thinking about that a lot while watching agent code. We are doing
the handwritten-parser thing again, except the untrusted thing is not a JSON
payload from a client. It is a generative model deciding what the program
should do next.

## What feels familiar about Jev

Jev is TypeSafe's decision model. The pitch is not "another chat wrapper." You
send some state and a few typed questions. You get structured answers plus
probabilities, and your code branches on those answers.

The analogy is not "it returns a Pydantic model." Pydantic already does that.
The analogy is the boundary move.

Today a lot of agent loops look like this:

```text
context -> generative LLM -> prompt engineering -> JSON parsing -> retries
```

Jev is trying to make that look more like:

```text
context -> declared decision space -> typed probabilistic decision
```

A sentence I keep coming back to:

Pydantic made structural uncertainty explicit. Jev is trying to make semantic
uncertainty explicit.

Pydantic answers: does this data match the program's structure?

Jev answers: given this messy information, which of the interpretations I
declared is more likely?

That is a different question. I think we have been stuffing both into
`await llm.generate(...)` because the LLM is sitting there and it will emit
tokens if we ask.

## Most of those tokens are not generation

A lot of Python agents currently treat the LLM as a universal computer:

- classify the request
- pick a tool
- decide if there is enough evidence
- judge whether a result is acceptable
- choose whether to continue
- write an explanation
- return JSON that pretends all of that was one thought

Some of those steps really are generation. Writing, synthesizing, explaining.
The rest are bounded decisions wearing a text-generation costume.

I like the split TypeSafe is pointing at, even if the product is still new:

```text
                    ,-- decision model: classify, rank, gate, route
application state --+
                    '-- generative LLM: reason, write, synthesize
```

The code change is easy to oversell. We have all written the first version:

```python
response = await llm.generate("Decide what to do and return JSON...")
decision = Decision.model_validate_json(response)
```

Pydantic is doing the second line. The first line is still "please emit
something that looks like my schema." If the model is a bit poetic, you retry.
If it picks a valid but wrong option, the type checker is happy and production
is not.

The second version is conceptually closer to TypeSafe's Python SDK, which
already asks questions as `Noul`, `Choice`, and `Score`:

```python
from typesafe_sdk import AsyncTypeSafeClient, Choice, Noul, Score

async with AsyncTypeSafeClient() as client:
    decision = await client.system_one(
        state=state,
        questions={
            "route": Choice(
                instructions="What should we do next?",
                criteria={
                    "search": None,
                    "code": None,
                    "answer": None,
                    "escalate": None,
                },
            ),
            "enough_evidence": Noul(
                instructions="Do we have enough evidence to answer?",
            ),
            "quality": Score(
                instructions="How good is this result?",
                criteria=["reject", "needs work", "good enough"],
            ),
        },
    )
```

The interesting part is not prettier syntax. It is that the model's native
object is a typed decision distribution, not a pile of tokens we later coerce
into a type.

That is the pydantic-shaped feeling. Correctness moves from scattered prompt
conventions into a reusable boundary.

## It sits on pydantic, literally

This is also not only a metaphor.

The official Python SDK (`typesafe-sdk`) depends on `pydantic>=2.12.0` and
`pydantic-core>=2.41.1`. Question objects inherit from `BaseModel`, serialize
through Pydantic, and forbid unexpected fields. Answers are strict, frozen
models with a discriminated union for `NoulAnswer | ChoiceAnswer | ScoreAnswer`.

So the stack is pretty honest:

```text
Pydantic
  └── checks that Jev's API contracts are well formed

Jev
  └── supplies a probabilistic judgment inside those contracts
```

Pydantic still owns structural validity. Jev owns the semantic guess.

Pydantic AI is starting to meet it halfway. `2.45` added a first-party
`TypeSafeModel` for Jev. `2.46` let that model fill tool arguments Jev can
express, pick among a union of output types, and take a configurable boolean
threshold. If you already live in that ecosystem, the boundary is showing up
inside the framework, not only as a side API.

## I would not call it the next pydantic yet

I want the idea. I do not want to overfit on the current vendor.

Pydantic became boring infrastructure because it was local, open source,
deterministic, and useful at almost every Python data edge. You could run it
without calling someone's model.

Jev is narrower right now. It is a hosted, proprietary decision model. It
handles bounded questions, not arbitrary schemas. It can still pick the wrong
valid answer. It does not replace writing or long reasoning. The calibration
story needs people besides TypeSafe to keep kicking the tires.

The larger thought may outlive Jev itself: decision models as a separate
software primitive from generative language models.

If that interface becomes something you can swap, a `DecisionModel` protocol
with choices, scores, boolean probabilities, calibration metadata, and a
policy for "don't act if confidence is junk," then it starts to look like a
pydantic-scale shift for agentic Python. If it stays one vendor's API, it is
still a useful tool, just not a new layer of the language.

I keep landing here:

Jev is not another agent framework. It is proposing a primitive underneath
those frameworks: declarative, typed semantic judgment. Pydantic formalized
data boundaries. This might be an early attempt to formalize decision
boundaries.

I am still chewing on it. If you have been wiring agents and also remember
the pydantic years, I would love to hear whether this click feels real or
whether I am mapping a new API onto a memory because the memory is comforting.

What do you think? What would a provider-independent decision primitive need
before you trusted it in a loop that spends money or pages someone?

## Related units

- [Productionizing agents](/etc/thoughts/agents/productionizing-agents)
- [Documenting for AI collaboration](/etc/thoughts/agents/documenting-for-ai-collaboration)
- [Building sustainable systems](/etc/thoughts/philosophy/sustainable-systems)

## References

- [TypeSafe docs: Jev / System One](https://docs.typesafe.ai/)
- [TypeSafe Python SDK](https://docs.typesafe.ai/sdk/python/)
- [typesafe-sdk pyproject.toml](https://github.com/typesafe-ai/typesafe-sdk-python/blob/main/pyproject.toml)
- [Question types](https://github.com/typesafe-ai/typesafe-sdk-python/blob/main/src/typesafe_sdk/_core/question_types.py)
- [Response types](https://github.com/typesafe-ai/typesafe-sdk-python/blob/main/src/typesafe_sdk/_core/response_types.py)
- [Pydantic AI v2.45.0](https://github.com/pydantic/pydantic-ai/releases/tag/v2.45.0)
- [Pydantic AI v2.46.0](https://github.com/pydantic/pydantic-ai/releases/tag/v2.46.0)
