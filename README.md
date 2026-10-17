# lib-dedent

Dedent multi-line text — the `dedent` library of the Handl platform.

`src/dedent.hnl` exports a single arrow: it normalizes line endings, strips
surrounding blank lines, finds the largest leading whitespace prefix common to
all non-blank lines, and removes it while keeping relative indentation intact.

Part of the `handl-lang` org split (ADR-0128). History is a `git subtree
split` of `platform/handl-packages/dedent/`.

## Layout

```text
src/        the .hnl sources
pkg.hnl     dependencies, when the package has any:
            `{ deps: { <name>: "branch:<ref>" } }`
```

`dedent` imports `str`, which comes from the module chain, so it declares no
dependencies and has no `pkg.hnl`.

## Gate

The `.hnl` sources are not covered by `cargo`; `hnl typecheck` / `lint` /
`test` is their gate, run in CI and available locally:

```bash
HNL=../cli/target/debug/hnl scripts/hnl-gate.sh
```
