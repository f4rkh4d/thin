# thin

[![ci](https://github.com/f4rkh4d/thin/actions/workflows/ci.yml/badge.svg)](https://github.com/f4rkh4d/thin/actions/workflows/ci.yml)
[![license](https://img.shields.io/github/license/f4rkh4d/thin)](https://github.com/f4rkh4d/thin/blob/main/LICENSE)

![demo](docs/hero.gif)

finds ai writing in your readmes. without using ai.

rule-based prose linter. opinionated. loud.

## install

```
cargo install --git https://github.com/f4rkh4d/thin
```

brew and curl installers coming. for now, cargo is the path.

## quick start

```
thin README.md
```

run it on a glob:

```
thin "**/*.md"
```

read from stdin:

```
echo "it leverages synergy" | thin --stdin
```

## what it catches

~75 rules across 9 categories. a few highlights:

- **em-dash clusters**: 3+ em-dashes in a paragraph. the single loudest ai-prose tell.
- **empty adjectives**: `seamless`, `robust`, `blazingly fast`, `revolutionary`, `cutting-edge`.
- **filler verbs**: `leverages`, `utilizes`, `harnesses the power of`, `empowers`.
- **filler openers**: `in today's fast-paced world`, `dive deep into`, `without further ado`.
- **parallel sophistry**: `not x. it's y.` / `not just x, but y.` / `it's not x, it's y.`
- **ai-signature phrases**: `delve`, `it's important to note`, `a testament to`, `in conclusion`.
- **passive clusters**: paragraphs where more than half of sentences are passive.
- **long sentences**: flagged at 35+ words, warned at 50+.
- **redundancy**: `end result`, `past history`, `in order to`.
- **corporate-speak**: `synergy`, `move the needle`, `mission-critical`.

run `thin --list-rules` for the full set. `thin --rule <id>` for a bad/good pair.

## presets

```
thin --profile balanced   # default
thin --profile frkhd      # everything is an error
thin --profile relaxed    # only the loudest tells
thin --profile corporate  # ironic. lets corporate-speak through.
```

## config

drop a `thin.toml` next to your project:

```toml
[profile]
name = "balanced"

[rules]
"thin.filler.leverages" = { severity = "error" }
"thin.passive.cluster" = "off"

[ignore]
paths = ["vendor/", "node_modules/"]
patterns = ["<!-- thin: ignore-file -->"]
```

## inline ignores

```
<!-- thin: ignore-line -->
<!-- thin: ignore-next -->
<!-- thin: ignore-rule thin.em-dash.cluster -->
```

## --fix

a few rules have deterministic replacements. these get auto-fixed in place:

- `utilizes` → `uses`
- `in order to` → `to`
- `leverages` → `uses`
- `end result` → `result`
- `past history` → `history`

```
thin --fix README.md
```

subjective things (parallel sophistry, long sentences, passive voice) are never auto-fixed. thin prints the flag; you decide.

## faq

**is this just a bunch of regex?** yes, but well-organized.

**will it flag my deliberate style?** probably. add `<!-- thin: ignore-line -->` or go lint somewhere else.

**does it use an llm?** no. the joke writes itself.

**will it catch humans writing in an ai style?** yes. that's the point. if it sounds generated, it probably shouldn't be shipped.

**what about multilingual support?** v0.1 is english. not planned soon.

**why rust?** because i couldn't find `blazingly fast` without flagging myself.

## vs others

- **vale**. vale is great and exhaustive. thin is smaller, more opinionated, specifically targets ai-prose tells.
- **write-good**. similar idea, older, not maintained.
- **lt / grammarly**. those catch grammar. thin catches style.

## roadmap

- v0.2: lsp code actions, editor plugins.
- v0.3: multilingual.
- later: custom rule sdk.

lsp is deferred to v0.2 to keep v0.1 small.

## license

mit.
