---
doc_id: AE-06-03
title: Build Systems
status: active
last_verified: 2026-05-22
source_scope: build-configuration-for-performance-work
depends_on:
  - AE-05-03
see_also:
  - AE-06-02
  - AE-06-05
  - AE-91-01
---

# Build Systems

## Scope
Operational guide to build-system configuration for reproducible performance work across CMake, Meson, and Cargo.

## Decision Matrix
| System | Key feature | Performance relevance |
| --- | --- | --- |
| CMake | Presets and workflows | Share exact configure/build/test settings. |
| Meson | Built-in optimization/debug/LTO options | Make tuning explicit and inspectable. |
| Cargo | Profiles and per-profile overrides | Control optimization, LTO, debug info, and codegen units. |

## Policy
- Build settings are part of the experiment definition; compiler flags, LTO, debug info, and profile mode must be versioned or recorded. [SRC-040] [SRC-041] [SRC-042]
- Use preset or profile names as experiment handles so results can be reproduced without reconstructing command lines. [SRC-041] [SRC-040]

## Recommended Practice
- CMake: use checked-in CMakePresets.json for shared configurations and keep user-local variation in CMakeUserPresets.json. [SRC-041]
- Meson: drive optimization through built-in options such as buildtype, b_lto, debug, and warning level; avoid hidden environment-only tuning. [SRC-042]
- Cargo: define named profiles for benchmark and release variants instead of overloading one release configuration. [SRC-040]

## Failure Modes
- Comparing performance across branches without aligning profile and preset semantics invalidates results quickly.
- Embedding local machine assumptions in shared build files makes cross-host reproduction harder, not easier.

## Related Docs
- [AE-06-02: Benchmark Frameworks](02-benchmark-frameworks.md)
- [AE-06-05: Performance Workflows](05-performance-workflows.md)
- [AE-91-01: Document Template](../_templates/01-document-template.md)

## Sources used
- [SRC-040](https://doc.rust-lang.org/cargo/reference/profiles.html) - Profiles. Cargo reference. 2026-05-18 verified.
- [SRC-041](https://cmake.org/cmake/help/latest/manual/cmake-presets.7.html) - cmake-presets(7). CMake documentation. 2026-05-22 verified.
- [SRC-042](https://mesonbuild.com/Builtin-options.html) - Meson built-in options. Meson documentation. 2026-05-08 verified.
