# Pointer Layout Artifacts

This directory is the artifact root for the tightened pointer-layout family.

## Primary Documents

| file | purpose |
| --- | --- |
| [2026-05-22-container-perf-analysis.md](./2026-05-22-container-perf-analysis.md) | canonical interpretation of the first full Docker perf sweep |
| [docker-perf-summary.md](./docker-perf-summary.md) | machine-generated Markdown comparison from the latest Docker perf sweep |
| [docker-perf-summary.json](./docker-perf-summary.json) | machine-generated structured summary for downstream processing |
| [pmc-status.txt](./pmc-status.txt) | status of native Windows PMC/WPT attempts on this host |

## Raw Perf Outputs

| file | variant |
| --- | --- |
| [boxed_pointer_chain-perf-stat.txt](./boxed_pointer_chain-perf-stat.txt) | allocator scatter + pointer dependency |
| [arena_pointer_chain-perf-stat.txt](./arena_pointer_chain-perf-stat.txt) | contiguous allocation + pointer dependency |
| [packed_index_chase-perf-stat.txt](./packed_index_chase-perf-stat.txt) | contiguous allocation + integer dependency + packed node |
| [split_soa_index_chase-perf-stat.txt](./split_soa_index_chase-perf-stat.txt) | contiguous allocation + integer dependency + split arrays |

## Probe Outputs

| file | purpose |
| --- | --- |
| [boxed_pointer_chain-probe.json](./boxed_pointer_chain-probe.json) | standalone timing and footprint report |
| [arena_pointer_chain-probe.json](./arena_pointer_chain-probe.json) | standalone timing and footprint report |
| [packed_index_chase-probe.json](./packed_index_chase-probe.json) | standalone timing and footprint report |
| [split_soa_index_chase-probe.json](./split_soa_index_chase-probe.json) | standalone timing and footprint report |

## Windows PMC Notes

Native Windows PMC/WPT attempts are summarized in [pmc-status.txt](./pmc-status.txt).

Raw ETL traces and derived text exports are intentionally treated as local-only artifacts and are not part of the intended public repository history. The preferred committed evidence path is the Dockerized Linux `perf stat` output plus the curated summary documents.

## Recommended Reading Order

1. [2026-05-22-container-perf-analysis.md](./2026-05-22-container-perf-analysis.md)
2. [docker-perf-summary.md](./docker-perf-summary.md)
3. variant-specific `*-perf-stat.txt`
4. [pmc-status.txt](./pmc-status.txt)
