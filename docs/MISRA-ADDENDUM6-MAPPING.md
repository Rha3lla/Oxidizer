# MISRA-C:2025 Addendum 6 — Rust Applicability Mapping

This document tracks the applicability and implementation status of MISRA-C:2025 guidelines as they apply to Rust, per Addendum 6.

## Document Info

| Field | Value |
|-------|-------|
| MISRA-C Version | 2025 |
| Addendum | 6 (Applicability to Rust) |
| Last Updated | 2026-02-28 |
| Tool | HAAT (High-Assurance Analysis Tool) |

---

## Legend
### Rationale Codes (from MISRA)

### Rust Applicability Status
| Code | Meaning | Typical Severity Impact |
|------|---------|------------------------|
| UB | Undefined Behavior in C | Critical - often applies to unsafe Rust |
| IDB | Implementation-defined Behavior | High/Medium |
| CQ | Code Quality | Low/Medium |
| DC | Developer Confusion | Low/Medium |


### Rust Applicability (from Addendum 6)
| Status | Meaning |
|--------|---------|
| Yes | Applies equally to Rust |
| No | Does not apply to Rust |
| Partial | Partially applies to some aspects |


### Rust Category (from Addendum 6)
| Category | Meaning |
|----------|---------|
| Required | Shall comply; formal deviation required if not |
| Advisory | Should follow; document non-compliance |
| Disapplied | Compliance not required |
| N/A | Behavior does not apply to Rust |


### HAAT Severity (Our Assignment)
| Severity | Criteria |
|----------|----------|
| Critical | Undefined behavior, memory safety, exploitable vulnerability |
| High | Likely security/safety impact, data integrity risk |
| Medium | Potential issue under specific conditions |
| Low | Code quality, maintainability, best practice |


### HAAT Implementation Status
| Status | Meaning |
|--------|---------|
| ➖ Skip | Language-enforced or N/A — no implementation needed |
| ⬜ Backlog | Not yet scheduled |
| 🚧 Planned | In roadmap |
| 🔨 Active | Currently implementing |
| ✅ Done | Implemented and tested |


---
## Implementation Priority
Focus order for HAAT development:
1. **P1 - Critical/High + Required**: Must implement first
2. **P2 - Medium + Required**: Second wave
3. **P3 - Advisory (any severity)**: Third wave
4. **P4 - Safe-Rust-only concerns**: Polish phase
---

---
## Implementation Priority
Focus order for HAAT development:
1. **P1 - Critical/High + Required**: Must implement first
2. **P2 - Medium + Required**: Second wave
3. **P3 - Advisory (any severity)**: Third wave
4. **P4 - Safe-Rust-only concerns**: Polish phase
---
## Directives
| ID | Rationale | Rust General | Safe Rust | Rust Category | Severity | Priority | HAAT Status | Notes |
|----|-----------|--------------|-----------|---------------|----------|----------|-------------|-------|
| D.1.1 | IDB | Yes | Yes | Required | Medium | P2 | ⬜ Backlog | Implementation-defined behavior documentation |
| D.1.2 | IDB | Yes | Yes | Required | High | P1 | ⬜ Backlog | Experimental/unstable features must be documented |
| D.2.1 | UB, CQ, DC | Yes | Yes | Required | Critical | P1 | ⬜ Backlog | Assembly language usage |
| D.3.1 | CQ | Yes | Yes | Required | Low | P3 | ⬜ Backlog | Code comprehension |
| D.4.1 | UB, CQ | Yes | Yes | Required | Critical | P1 | ⬜ Backlog | Run-time failures (panics in Rust) |
| D.4.2 | IDB, CQ | Yes | No | Required | Medium | P2 | ⬜ Backlog | Unsafe code documentation |
| D.4.3 | DC, CQ | Yes | No | Advisory | Low | P3 | ⬜ Backlog | Assembly documentation |
| D.4.4 | DC | Yes | Yes | Advisory | Low | P3 | ⬜ Backlog | Conditional compilation (cfg attribute) |
| D.4.5 | DC | Yes | Yes | Advisory | Low | P3 | ⬜ Backlog | Ambiguous identifiers (project-defined) |
| D.4.6 | DC | No | No | N/A | — | — | ➖ Skip | Rust primitive types already fulfill this |
| D.4.7 | DC | Yes | Yes | Advisory | Medium | P3 | ⬜ Backlog | Error handling; prefer Option/Result |
| D.4.8 | DC | No | No | N/A | — | — | ➖ Skip | Not applicable |
| D.4.9 | DC, CQ | Yes | Yes | Advisory | Low | P3 | ⬜ Backlog | Function-like macros |
| D.4.10 | UB, DC | No | No | N/A | — | — | ➖ Skip | Header guards — not applicable |
| D.4.11 | UB, IDB | Yes | Yes | Advisory | Medium | P3 | ⬜ Backlog | Validity of parameters |
| D.4.12 | UB, CQ | Yes | Yes | Advisory | High | P2 | ⬜ Backlog | Dynamic memory |
| D.4.13 | UB, DC | Yes | Yes | Required | High | P1 | ⬜ Backlog | Resource ordering (Rust APIs use type system) |
| D.4.14 | UB, CQ | Yes | Yes | Required | Critical | P1 | ⬜ Backlog | External input validation |
| D.4.15 | UB, IDB, DC | Yes | Yes | Required | High | P1 | ⬜ Backlog | Floating-point (Rust implements IEEE-754) |
| D.5.1 | UB | Yes | Partial | Required | Critical | P1 | ⬜ Backlog | Data races — not all safe Rust types are race-free |
| D.5.2 | UB | Yes | Yes | Required | Critical | P1 | ⬜ Backlog | Thread safety |
| D.5.3 | UB, DC | Yes | Yes | Required | Critical | P1 | ⬜ Backlog | Concurrent access |
### Directive Summary
| Priority | Count | Description |
|----------|-------|-------------|
| P1 | 10 | Critical/High + Required |
| P2 | 2 | Medium + Required |
| P3 | 7 | Advisory |
| Skip | 4 | N/A for Rust |
| **Total** | **23** | |
---
## Rules
### Rules That Apply to Rust
These require HAAT implementation:
| ID | Rationale | Rust General | Safe Rust | Rust Category | Severity | Priority | HAAT Status | Notes |
|----|-----------|--------------|-----------|---------------|----------|----------|-------------|-------|
| R.1.1 | UB, IDB | Yes | Partial | Required | Critical | P1 | ⬜ Backlog | Conforming code |
| R.1.3 | UB, IDB | Yes | Yes | Required | Critical | P1 | ⬜ Backlog | Undefined/unspecified behavior |
| R.1.5 | UB, IDB, DC | Yes | Yes | Required | High | P1 | ⬜ Backlog | Deprecated APIs |
| R.2.1 | DC | Yes | Yes | Advisory | Low | P3 | ⬜ Backlog | Unreachable code |
| R.2.2 | DC | Yes | Yes | Required | Medium | P2 | ⬜ Backlog | Dead code |
| R.2.3 | DC | Yes | Yes | Advisory | Low | P3 | ⬜ Backlog | Unused type declarations |
| R.2.5 | DC | Yes | Yes | Advisory | Low | P3 | ⬜ Backlog | Unused macro declarations |
| R.2.6 | DC | Yes | Yes | Advisory | Low | P3 | ⬜ Backlog | Unused label declarations |
| R.2.7 | DC | Yes | Yes | Advisory | Low | P3 | ⬜ Backlog | Unused parameters |
| R.2.8 | DC | Yes | Yes | Advisory | Low | P3 | ⬜ Backlog | Unused variables |
| R.3.1 | DC | Yes | Yes | Advisory | Low | P3 | ⬜ Backlog | Comments — nested supported in Rust |
| R.5.1 | UB, IDB, DC | Yes | Partial | Required | Medium | P2 | ⬜ Backlog | Identifier length (extern "C" only) |
| R.5.2 | UB, IDB, CQ | Yes | Yes | Required | Medium | P2 | ⬜ Backlog | Identifier uniqueness |
| R.5.3 | DC | Yes | Yes | Advisory | Low | P3 | ⬜ Backlog | Identifier hiding (includes macros) |
| R.5.5 | UB, IDB, DC | Partial | Partial | Advisory | Low | P3 | ⬜ Backlog | Macro/identifier confusion |
| R.5.6 | DC | Yes | Yes | Advisory | Low | P3 | ⬜ Backlog | Type name uniqueness |
| R.5.8 | DC | Yes | Yes | Advisory | Low | P3 | ⬜ Backlog | Object/function uniqueness |
| R.5.9 | DC | Yes | Yes | Advisory | Low | P3 | ⬜ Backlog | Internal linkage uniqueness |
| R.5.10 | UB, DC | Yes | Partial | Advisory | Medium | P3 | ⬜ Backlog | Reserved identifiers |
| R.7.1 | DC | Yes | Yes | Advisory | Low | P3 | ⬜ Backlog | Octal constants |
| R.7.2 | DC | Yes | Yes | Advisory | Low | P3 | ⬜ Backlog | Unsigned suffix |
| R.8.3 | UB, DC | Yes | No | Required | Critical | P1 | ⬜ Backlog | extern declaration type compatibility |
| R.8.5 | DC | Yes | No | Advisory | Medium | P3 | ⬜ Backlog | extern "C" declarations |
| R.8.6 | UB | Yes | No | Required | High | P1 | ⬜ Backlog | extern "C" definitions |
| R.8.7 | DC | Yes | Yes | Advisory | Low | P3 | ⬜ Backlog | pub visibility minimization |
| R.8.9 | DC | Yes | Yes | Advisory | Low | P3 | ⬜ Backlog | Variable scope minimization |
| R.8.13 | DC | Yes | Yes | Advisory | Low | P3 | ⬜ Backlog | mut minimization |
| R.8.15 | UB | Yes | No | Required | High | P1 | ⬜ Backlog | extern "C" layout |
| R.8.17 | DC | Partial | Partial | Advisory | Low | P3 | ⬜ Backlog | Alignment |
| R.9.1 | UB | Yes | Yes | Required | Critical | P1 | ⬜ Backlog | Initialization — rustc enforces, unsafe can bypass |
| R.9.4 | DC | Yes | Yes | Required | Low | P2 | ⬜ Backlog | Duplicate initialization — rustc enforces |
| R.9.7 | UB | Yes | No | Required | Critical | P1 | ⬜ Backlog | Uninitialized memory |
| R.10.5 | DC | Yes | Partial | Advisory | Medium | P3 | ⬜ Backlog | Casts (as and transmute) |
| R.10.8 | DC | Yes | Partial | Advisory | Medium | P3 | ⬜ Backlog | Cast width changes |
| R.11.1 | UB, IDB | Yes | Partial | Required | Critical | P1 | ⬜ Backlog | Pointer/function casts |
| R.11.2 | UB | Yes | No | Required | Critical | P1 | ⬜ Backlog | Incomplete type pointer casts |
| R.11.3 | UB | Yes | Yes | Required | Critical | P1 | ⬜ Backlog | Pointer type casts |
| R.11.4 | UB, IDB | Yes | Yes | Required | Critical | P1 | ⬜ Backlog | Pointer/integer casts |
| R.11.5 | UB | Yes | No | Required | Critical | P1 | ⬜ Backlog | void pointer casts |
| R.11.6 | UB, IDB | Yes | No | Advisory | High | P2 | ⬜ Backlog | Pointer/integer roundtrip |
| R.11.8 | UB | Yes | Partial | Required | Critical | P1 | ⬜ Backlog | const cast removal |
| R.11.11 | DC | Yes | Yes | Required | Low | P2 | ⬜ Backlog | Function pointer casts — rustc enforces |
| R.12.1 | DC | Yes | Yes | Advisory | Low | P3 | ⬜ Backlog | Operator precedence |
| R.12.2 | UB, DC | Yes | Partial | Advisory | High | P3 | ⬜ Backlog | Shift operator range |
| R.12.4 | DC | Yes | No | Advisory | Medium | P3 | ⬜ Backlog | Constant expressions |
| R.13.1 | UB | Yes | Yes | Advisory | Medium | P3 | ⬜ Backlog | Initializer side effects |
| R.13.5 | DC | Yes | Yes | Advisory | Low | P3 | ⬜ Backlog | Side effects in && / \|\| |
| R.14.1 | DC | Yes | Partial | Required | Medium | P2 | ⬜ Backlog | Loop counters (while loops) |
| R.14.3 | DC | Yes | Yes | Required | Medium | P2 | ⬜ Backlog | Invariant boolean |
| R.14.4 | DC | Yes | Yes | Required | Low | P2 | ⬜ Backlog | Boolean conditions — rustc enforces |
| R.15.4 | DC | Yes | Yes | Advisory | Low | P3 | ⬜ Backlog | Loop termination |
| R.15.5 | DC | Yes | Yes | Disapplied | — | — | ➖ Skip | Single exit point — disapplied for Rust |
| R.15.7 | DC | Yes | Yes | Advisory | Low | P3 | ⬜ Backlog | if-else chains |
| R.17.2 | UB, DC | Yes | Yes | Required | High | P1 | ⬜ Backlog | Recursion |
| R.17.7 | DC | Yes | Yes | Required | Medium | P2 | ⬜ Backlog | Return value usage (must_use) |
| R.17.8 | DC | Yes | Yes | Disapplied | — | — | ➖ Skip | Parameter modification — disapplied |
| R.17.9 | UB | Yes | No | Required | Critical | P1 | ⬜ Backlog | Non-returning functions (! type) |
| R.17.11 | DC | Yes | Yes | Advisory | Low | P3 | ⬜ Backlog | Non-returning with return type |
| R.18.1 | UB | Yes | No | Required | Critical | P1 | ⬜ Backlog | Pointer bounds |
| R.18.2 | UB | Yes | No | Required | Critical | P1 | ⬜ Backlog | Pointer subtraction |
| R.18.3 | UB | Yes | Yes | Required | Critical | P1 | ⬜ Backlog | Pointer comparison |
| R.18.4 | DC | Yes | No | Advisory | Medium | P3 | ⬜ Backlog | Pointer arithmetic |
| R.18.5 | DC | Yes | Yes | Advisory | Low | P3 | ⬜ Backlog | Pointer indirection levels |
| R.18.6 | UB | Yes | No | Required | Critical | P1 | ⬜ Backlog | Dangling pointers |
| R.19.1 | UB | Yes | No | Required | Critical | P1 | ⬜ Backlog | Object overlap |
| R.19.2 | UB, DC | Yes | Yes | Advisory | High | P2 | ⬜ Backlog | Union usage |
| R.19.3 | UB | Yes | Yes | Required | Critical | P1 | ⬜ Backlog | Object representation |
| R.20.4 | UB | Partial | Partial | Required | Medium | P2 | ⬜ Backlog | Reserved identifiers (raw identifiers) |
| R.20.7 | DC | Partial | Partial | Advisory | Low | P3 | ⬜ Backlog | Macro parameters (proc macros only) |
| R.21.3 | UB, IDB | Yes | No | Required | Critical | P1 | ⬜ Backlog | malloc/free via FFI |
| R.21.4 | UB | Yes | No | Required | Critical | P1 | ⬜ Backlog | setjmp via FFI |
| R.21.5 | UB, IDB | Yes | No | Required | Critical | P1 | ⬜ Backlog | signal via FFI |
| R.21.6 | UB, IDB | Yes | No | Required | High | P1 | ⬜ Backlog | stdio via FFI |
| R.21.7 | UB | Yes | No | Required | Critical | P1 | ⬜ Backlog | atof/atoi via FFI |
| R.21.8 | UB, IDB | Yes | No | Required | Critical | P1 | ⬜ Backlog | abort/exit via FFI |
| R.21.9 | UB | Yes | No | Required | Critical | P1 | ⬜ Backlog | getenv via FFI |
| R.21.10 | UB, IDB | Yes | No | Required | High | P1 | ⬜ Backlog | time functions via FFI |
| R.21.12 | UB, IDB | Yes | No | Required | High | P1 | ⬜ Backlog | fenv exceptions via FFI |
| R.21.13 | UB | Yes | No | Required | Critical | P1 | ⬜ Backlog | ctype.h via FFI |
| R.21.14 | DC | Yes | No | Required | Medium | P2 | ⬜ Backlog | memcmp on strings via FFI |
| R.21.15 | DC | Yes | No | Required | High | P2 | ⬜ Backlog | memcpy/memmove types via FFI |
| R.21.16 | UB | Yes | No | Required | Critical | P1 | ⬜ Backlog | memcmp types via FFI |
| R.21.17 | UB | Yes | No | Required | Critical | P1 | ⬜ Backlog | String function bounds via FFI |
| R.21.18 | UB | Yes | No | Required | Critical | P1 | ⬜ Backlog | size_t arguments via FFI |
| R.21.19 | UB | Yes | No | Required | Critical | P1 | ⬜ Backlog | localeconv/getenv pointers via FFI |
| R.21.20 | IDB, DC | Yes | No | Required | High | P1 | ⬜ Backlog | setlocale/strerror pointers via FFI |
| R.21.21 | UB, IDB | Yes | No | Required | Critical | P1 | ⬜ Backlog | system() via FFI |
| R.21.24 | CQ | Yes | No | Required | Medium | P2 | ⬜ Backlog | rand() via FFI |
| R.21.25 | UB | Yes | Yes | Required | Critical | P1 | ⬜ Backlog | Invalid memory access |
| R.21.26 | UB | Yes | No | Required | Critical | P1 | ⬜ Backlog | Thread-local errno via FFI |
| R.22.1 | UB, CQ | Yes | No | Required | Critical | P1 | ⬜ Backlog | Resource cleanup via FFI |
| R.22.2 | UB | Yes | No | Required | Critical | P1 | ⬜ Backlog | Memory free only if allocated via FFI |
| R.22.3 | UB, IDB | Yes | No | Required | High | P1 | ⬜ Backlog | File mode consistency via FFI |
| R.22.4 | UB | Yes | No | Required | Critical | P1 | ⬜ Backlog | File read/write on closed file via FFI |
| R.22.5 | IDB | Yes | No | Required | High | P1 | ⬜ Backlog | FILE pointer dereference via FFI |
| R.22.6 | UB | Yes | No | Required | Critical | P1 | ⬜ Backlog | FILE after fclose via FFI |
| R.22.7 | DC | Yes | No | Required | Medium | P2 | ⬜ Backlog | EOF macro via FFI |
| R.22.8 | DC | Yes | No | Disapplied | — | — | ➖ Skip | errno before call via FFI |
| R.22.9 | DC | Yes | No | Disapplied | — | — | ➖ Skip | errno after call via FFI |
| R.22.10 | DC | Yes | No | Disapplied | — | — | ➖ Skip | errno checking via FFI |
| R.22.11 | UB | Yes | Partial | Required | Critical | P1 | ⬜ Backlog | Thread creation resources |
| R.22.12 | UB | Yes | No | Required | Critical | P1 | ⬜ Backlog | Thread join/detach via FFI |
| R.22.13 | UB, DC | Yes | Yes | Required | Critical | P1 | ⬜ Backlog | Thread ID usage |
| R.22.14 | UB | Yes | Partial | Required | Critical | P1 | ⬜ Backlog | Sync object creation before threads |
| R.22.15 | UB | Yes | Partial | Required | Critical | P1 | ⬜ Backlog | Sync object release after threads |
| R.22.16 | UB | Yes | No | Required | Critical | P1 | ⬜ Backlog | Mutex unlock via FFI |
| R.22.17 | UB | Yes | No | Required | Critical | P1 | ⬜ Backlog | Mutex non-recursive via FFI |
| R.22.18 | UB | Yes | Yes | Required | Critical | P1 | ⬜ Backlog | Mutex double-lock |
| R.22.19 | UB | Yes | Yes | Required | Critical | P1 | ⬜ Backlog | Mutex unlock ownership |
| R.22.20 | UB | Yes | Partial | Required | Critical | P1 | ⬜ Backlog | Thread execution order |
### Rules That Do NOT Apply to Rust
These can be skipped entirely (N/A):
| ID | Reason |
|----|--------|
| R.1.4 | C versioning specific |
| R.2.4 | No separate tag namespace |
| R.3.2 | Line splicing |
| R.4.1, R.4.2 | Escape sequences |
| R.5.4 | Macro uniqueness |
| R.5.7 | Tag namespace |
| R.6.1-R.6.3 | Bit-field specific |
| R.7.3-R.7.6 | Literal suffixes/encoding |
| R.8.1-R.8.2, R.8.4 | Declaration specifics |
| R.8.8, R.8.10-R.8.12 | Static/extern specifics |
| R.8.14, R.8.16, R.8.18-R.8.19 | Declaration specifics |
| R.9.2-R.9.3, R.9.5-R.9.6 | Initialization specifics |
| R.10.1-R.10.4, R.10.6-R.10.7 | Type conversions |
| R.11.9-R.11.10 | Null pointer constant |
| R.12.3, R.12.5-R.12.6 | Expression specifics |
| R.13.2-R.13.4, R.13.6 | Evaluation order (Rust is strict) |
| R.14.2 | For loop specifics |
| R.15.1-R.15.3, R.15.6 | goto/switch |
| R.16.1-R.16.7 | Switch statement |
| R.17.1, R.17.3-R.17.5 | Function specifics |
| R.17.10, R.17.12-R.17.13 | Function specifics |
| R.18.7-R.18.10 | Array/pointer specifics |
| R.20.1-R.20.3, R.20.5-R.20.6 | Preprocessor |
| R.20.8-R.20.15 | Preprocessor |
| R.21.11, R.21.22-R.21.23 | No external interface |
| R.23.1-R.23.8 | Generic selection |
---
## Priority Summary
| Priority | Count | Focus Area |
|----------|-------|------------|
| **P1** | ~45 | Critical/High severity + Required — implement first |
| **P2** | ~12 | Medium severity + Required |
| **P3** | ~20 | Advisory rules |
| **Skip** | ~90 | N/A or Disapplied |
| **Total** | ~167 | |
---
## Implementation Phases
### Phase 1: Safe Rust Fundamentals
Rules that apply to all Rust code (Safe Rust = Yes):
- D.4.1, D.4.14, D.5.1-D.5.3 (runtime failures, concurrency)
- R.9.1 (initialization)
- R.17.2 (recursion)
- R.21.25 (memory access)
- R.22.13, R.22.18-R.22.20 (threading)
### Phase 2: Unsafe Rust / FFI
Rules that apply when using unsafe or extern "C":
- R.8.3, R.8.6, R.8.15 (extern declarations)
- R.11.x (pointer casts)
- R.18.x (pointer operations)
- R.21.x (C library functions)
- R.22.x (resource management)
### Phase 3: Code Quality
Advisory rules for maintainability:
- R.2.x (dead code)
- R.5.x (naming)
- R.8.7, R.8.9, R.8.13 (scope/visibility)
