# site

The KungFu project page. Static HTML and CSS — no build step, no dependencies, no JavaScript.

```
site/
  index.html    the page
  style.css     design system
  logo.svg      mark — three streams converging into one
```

## Local preview

```
python3 -m http.server 8000 --directory site
```

## Design

Shares its design system with [SuperX](https://k8nstantin.github.io/superx/) — the same
palette, type scale, spacing tokens and components, so the two projects read as one family.
Space Grotesk for headings, JetBrains Mono for code, `#1F062A` ground with the purple accent
ramp.

KungFu-specific components live in the `KungFu additions` block at the end of `style.css`:

| class | what it is |
|---|---|
| `.flow` / `.flow__state` | the five-state pipeline, with `--gate` for the machine gate |
| `.grid-table` | the three-gates and foundations tables |
| `.ledger-pair` / `.ledger` | the gone/kept and team/company two-ups |
| `.quote` | pull quotes |
| `.limit` | the "what we don't claim" entries |

If the palette or a shared component changes upstream, copy it down rather than diverging.

## Publishing

GitHub Pages, served from `/site` on the default branch — Settings → Pages → Source:
`main` / `/site`.

## Editing

The page follows the README and should not drift from it. Two rules carry over:

- **Every claim carries its mechanism.** No assertion without the reason underneath it.
- **The "what we don't claim" section stays.** It is the most credible part of the page and
  the first thing an experienced engineer looks for.
