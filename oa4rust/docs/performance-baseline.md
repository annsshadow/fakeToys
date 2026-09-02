# Performance Baseline & Comparison Report

- **Rust backend**: `http://localhost:3000`
- **Java backend**: SKIP (unreachable)
- **Concurrency**: 549 (derived from requests)

## Rust Baseline

| Scenario | Total | Success | Errors | Error Rate | QPS | P50 (ms) | P95 (ms) | P99 (ms) |
|----------|-------|---------|--------|------------|-----|-----------|-----------|-----------|
| login | 549 | 0 | 549 | 100.00% | 0.16 | 7913.49 | 8175.16 | 8189.10 |
| work-start | 0 | 0 | 0 | 0.00% | 0.00 | 0.00 | 0.00 | 0.00 |
| cms | 0 | 0 | 0 | 0.00% | 0.00 | 0.00 | 0.00 | 0.00 |

## Java (o2server) Baseline

Java backend was not reachable. Comparison skipped.

## Environment

- **Date**: 395806.9360261
- **Tool**: aiohttp + asyncio (wrk/ab not available in this environment)
