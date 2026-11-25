# lib-dedent

Dedent multi-line text — the `dedent` library of the Handl platform.

`src/dedent.hnl` exports a single arrow: it normalizes line endings, strips
surrounding blank lines, finds the largest leading whitespace prefix common to
all non-blank lines, and removes it while keeping relative indentation intact.

## Layout

```text
src/                     the .hnl sources
pkg.hnl                  package manifest
scripts/hnl-gate.sh      the gate
workflows/gate.yaml                    bupha entry (handl-gates)
```

## Gate

The `.hnl` sources are not covered by `cargo`; `hnl typecheck` / `lint` /
`test` is their gate, run in CI and available locally:

```bash
HNL=../host-cli/target/debug/hnl scripts/hnl-gate.sh
```

## License

MIT.
