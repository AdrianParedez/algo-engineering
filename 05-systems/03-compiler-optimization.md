---
doc_id: AE-05-03
title: Compiler Optimization
status: active
last_verified: 2026-05-22
source_scope: compiler-pipelines-and-performance-feedback
depends_on:
  - AE-01-03
see_also:
  - AE-03-02
  - AE-06-03
  - AE-07-02
---

# Compiler Optimization

## Scope
Applied algorithm engineering for compilers: IR choice, legality and profitability analysis, optimization remarks, affine transforms, and profile-guided decisions.

## Decision Matrix
| Layer | Primary question | Representative mechanism |
| --- | --- | --- |
| Frontend / early mid-end | Can the structure be exposed? | Canonical loops, alias info, simplified control flow. |
| Mid-end | Is transformation legal and profitable? | Vectorizers, inliners, loop passes. |
| Structured IR / MLIR | Can domain structure be preserved longer? | Affine, vector, and tensor dialect passes. |
| Feedback loop | What does runtime evidence change? | Optimization remarks and PGO. |

## Theory
- Compiler optimization is algorithm engineering on programs: transformations need legality proofs, cost models, and experimental validation.
- MLIR exists because many performance-critical transformations require structured information that low-level IR alone cannot recover cheaply. [SRC-012] [SRC-013]

## Production Reality
- Profitability heuristics are workload- and target-dependent; remarks and generated-code inspection are necessary because the cost model is never omniscient. [SRC-009] [SRC-010] [SRC-011]
- Affine passes such as tiling, fusion, and unroll-jam are powerful when access functions are regular enough to justify structured optimization. [SRC-014]

## Optimization Patterns
- Keep source or IR shapes compiler-friendly before reaching for manual intrinsics or handwritten kernels.
- Use optimization remarks as a regression interface: if a key loop stops vectorizing or inlining, treat that as a diagnosable event rather than a mystery. [SRC-010] [SRC-011]

## Failure Modes
- Blind trust in compiler magic leads to silent performance cliffs after small source or toolchain changes.
- Overly low-level manual tuning can destroy higher-level opportunities the compiler or runtime could have composed better.

## Related Docs
- [AE-03-02: SIMD Vectorization](../03-performance-engineering/02-simd-vectorization.md)
- [AE-06-03: Build Systems](../06-tooling/03-build-systems.md)
- [AE-07-02: Modern Breakthroughs](../07-research/02-modern-breakthroughs.md)

## Sources used
- [SRC-009](https://llvm.org/docs/Vectorizers.html) - Auto-Vectorization in LLVM. LLVM documentation. 2026-05-08 verified.
- [SRC-010](https://clang.llvm.org/docs/UsersManual.html) - Clang Compiler User’s Manual: Optimization Remarks. Clang documentation. 2026-05-16 verified.
- [SRC-011](https://gcc.gnu.org/onlinedocs/gcc-7.2.0/gcc/Developer-Options.html) - GCC Developer Options. GCC documentation. 2026-05-22 verified.
- [SRC-012](https://mlir.llvm.org/docs/Rationale/Rationale/) - MLIR Rationale. MLIR documentation. 2026-05-15 verified.
- [SRC-013](https://mlir.llvm.org/docs/Dialects/Affine/) - Affine Dialect. MLIR documentation. 2026-05-16 verified.
- [SRC-014](https://mlir.llvm.org/docs/Passes/) - MLIR Passes. MLIR documentation. 2026-05-08 verified.
