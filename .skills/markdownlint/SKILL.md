---
name: markdownlint
description: Run markdownlint on ThinkOS content and specs. Use after writing or editing markdown posts to prevent lint regressions (MD025, MD013, etc.).
license: MIT
compatibility: Requires Node.js and either markdownlint-cli2 or npx. Network may be required if using npx to install tools.
metadata:
  author: guilherme-am
  version: "1.0"
---

# markdownlint for thinkctl.dev

## When to use

- After editing any file under `content/etc/thoughts/**`
- After editing specs under `.cursor/specs/**`
- Before publishing changes

## Config

Uses repo config:

- `.markdownlint.json`
- `.markdownlintignore`

## Run

From repo root:

```bash
npx --yes markdownlint-cli2 "**/*.md"
```

Recommended scoped run:

```bash
npx --yes markdownlint-cli2 "content/etc/thoughts/**/*.md" ".cursor/specs/**/*.md" ".skills/**/*.md"
```

## Notes

- Prefer fixing markdown instead of disabling rules.
- Use inline disables only for exceptional cases:
  - `<!-- markdownlint-disable-next-line MD013 -->`

References:

- Agent Skills overview: `https://agentskills.io/home`
- markdownlint-cli docs: `https://github.com/igorshubovych/markdownlint-cli`
