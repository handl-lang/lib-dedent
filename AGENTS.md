# lib-dedent

`dedent` — dedent multi-line text: normalizes line endings, strips
surrounding blank lines, removes the largest leading-whitespace prefix
common to all non-blank lines while keeping relative indentation.

## Layout

| path | role |
| --- | --- |
| `src/index.hnl` | the library — exports the dedent arrow |
| `src/dedent.test.hnl` | tests |
| `pkg.hnl` | manifest — `modules: ["dedent"]` |
| `scripts/hnl-gate.sh` | `.hnl` gate |
| `workflows/gate.yaml` | CI — shared `handl-gates` campaign |

## Contract

- Pure `.hnl`, single exported arrow; no Rust crate.

## Verification

```bash
HNL=../host-cli/target/debug/hnl scripts/hnl-gate.sh
```

<!-- myco:agent-awareness -->

## Agent awareness

- Session start / any work discussion: run `myco status --prompt`
  before proposing next steps — know the board first.
- User describes new work → offer to capture it (`myco todo add`,
  `issue new`, `proposal new`). Don't wait to be asked.
- Picking up a task: `myco resume <ID>` is the briefing.
- Wrapping up: `myco handoff <ID>` anything resumable; keep statuses
  honest (claimed → in-progress → blocked/done).

<!-- /myco:agent-awareness -->
