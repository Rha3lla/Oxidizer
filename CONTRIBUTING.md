# Contributing to Rust Static Analyzer

This document captures how work gets done on this project. It exists so
decisions made today are still clear six months from now.

---

## Project structure

| Location | Purpose |
|----------|---------|
| GitHub Issues | All tasks, bugs, and rule implementations |
| GitHub Milestones | M0–M6 groupings with progress tracking |
| GitHub Projects board | Kanban view of current work |
| Notion (Rules database) | Rule specs, MISRA mapping, CWE refs, deviation guidance |
| Notion (Architecture docs) | Technical design decisions and rationale |

---

## Workflow (solo phase)

### Starting a task

1. Pick an issue from the **Ready** column on the project board
2. Move it to **In Progress**
3. Create a branch named after the issue:
   ```
   git checkout -b issue-7-cargo-workspace
   ```
   Format: `issue-{number}-{short-description}`

4. Do the work. Commit often using the commit format below.
5. When done, push to main:
   ```
   git push origin issue-7-cargo-workspace:main
   ```
   Or merge locally:
   ```
   git checkout main && git merge issue-7-cargo-workspace
   git push origin main
   ```
6. Close the issue manually on GitHub and move it to **Done** on the board.

> When collaborators join, this switches to branch → PR → review → merge.
> Branch protection on main will be enabled at that point.

---

## Commit message format

This project uses **Conventional Commits**: https://www.conventionalcommits.org

```
<type>(<scope>): <short summary>

[optional body]

[optional footer: Closes #issue-number]
```

### Types

| Type | When to use |
|------|-------------|
| `feat` | New functionality (new rule, new CLI flag, new output format) |
| `fix` | Bug fix |
| `chore` | Tooling, deps, CI, config — no production code change |
| `docs` | Documentation only |
| `test` | Adding or fixing tests |
| `refactor` | Code restructure with no behavior change |
| `perf` | Performance improvement |

### Scopes

Use the crate or area being changed:

| Scope | Meaning |
|-------|---------|
| `core` | rust-sa-core crate |
| `cli` | rust-sa-cli crate |
| `rules` | rust-sa-rules crate |
| `ci` | GitHub Actions workflows |
| `config` | Configuration schema/loading |
| `sarif` | SARIF output emitter |
| `docs` | Documentation files |

### Examples

```
feat(core): add Finding struct with Severity and Confidence enums

Closes #3

chore(ci): add GitHub Actions CI skeleton with build/test/clippy/fmt jobs

Closes #2

feat(rules): implement HA-RUST-0001 unsafe block without safety comment

Severity: Error
Profile: High Assurance, MISRA Rust
CWE: CWE-676
Closes #12

fix(core): handle missing rust-sa.toml gracefully with Config::default()

Closes #4

docs(rules): add deviation guidance to HA-RUST-0001

test(core): add SARIF round-trip test for empty findings list
```

---

## Rules workflow

When implementing a new rule:

1. Open a GitHub issue using the **Rule** issue template
2. Add the rule spec to the **Notion Rules database** — set status to `Planned`
3. Implement the rule in `crates/rust-sa-rules/src/`
4. Write positive fixture (FAIL case) and negative fixture (PASS case)
5. Update Notion rule status to `Implemented`
6. Write the rule doc in `docs/rules/RULE-ID.md`
7. Update Notion rule status to `Documented`
8. Verify `cargo test` passes — update Notion to `Tested`
9. Close the GitHub issue

---

## Engineering gates

Every commit to main must satisfy:

- [ ] `cargo build` passes with zero warnings
- [ ] `cargo test --all` passes
- [ ] `cargo clippy -- -D warnings` passes
- [ ] `cargo fmt --check` passes

Run all four locally before pushing:

```bash
cargo build && cargo test --all && cargo clippy -- -D warnings && cargo fmt --check
```

Or use the Makefile shortcut (once it exists):
```bash
make check
```

---

## Adding a new dependency

Before adding any crate, check:

1. **Is it maintained?** Check last release date and open issues on crates.io
2. **Is it in the RustSec advisory database?** Run `cargo audit` after adding
3. **Does it pull in unsafe code?** Run `cargo geiger` after adding
4. **Is it necessary?** Prefer stdlib solutions for simple problems

Document the decision in a commit message body if the dependency choice is
non-obvious.

---

## Versioning

This project follows **Semantic Versioning**: https://semver.org

| Milestone | Version |
|-----------|---------|
| M0 | 0.1.0 |
| M1 | 0.2.0 |
| M2 | 0.3.0 |
| M3 | 0.4.0 |
| M4 | 0.5.0 |
| M5 | 0.6.0 |
| M6 | 1.0.0-rc |

Bump the version in `Cargo.toml` when closing each milestone.

---

## Contact

Melissa Wilson — medicmommy83@gmail.com
