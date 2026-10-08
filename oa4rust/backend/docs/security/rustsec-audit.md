# RustSec dependency audit

OA4Rust uses one required advisory scanner and one policy scanner:

- `cargo-audit 0.22.2` owns RustSec vulnerability, unsound and yanked checks.
- `cargo-deny 0.20.2` owns bans, licenses and source provenance only.

Both scanners run with Rust 1.88.0 because their pinned releases require it. This does not change the application MSRV by itself.

## Reproducible pull-request gate

`.github/workflows/oa4rust-supply-chain.yml` reads the exact advisory database commit from `security/rustsec-db.rev`, verifies the checkout, prints the database SHA and `Cargo.lock` hash, and runs without fetching another advisory database:

```bash
cargo audit --file Cargo.lock \
  --db "$RUSTSEC_DB" --no-fetch --no-yanked --deny unsound
cargo deny --locked --workspace check bans licenses sources
```

The pinned gate uses `--no-yanked`: yanked state is maintained in the crates.io index rather than the RustSec repository, so claiming the result is reproducible without also pinning that index would be incorrect.

## Daily freshness gate

The scheduled job clones the current official RustSec database and runs the same scan with yanked checks enabled. Network failure is a hard failure; it never falls back to an old cache and does not use `--stale`.

## Updating the database pin

1. Verify the candidate against `https://github.com/RustSec/advisory-db`.
2. Replace `security/rustsec-db.rev` with the full 40-character commit SHA.
3. Run the pinned command locally with that exact checkout.
4. Review new advisories and remediate them before merging the pin update.

## Exceptions

The current policy has no advisory exceptions. A future exception must be time-limited and independently reviewed. At minimum it must record the advisory, package/version, dependency path, owner, tracking URL, creation and expiry dates, reason, compensating control, and approver. Expired or incomplete exceptions must fail CI; a bare advisory ID in `.cargo/audit.toml` is not an acceptable permanent exception.

## Failure policy

Vulnerabilities, unsound advisories and yanked dependencies fail their applicable job. Unmaintained dependencies remain visible warnings during the first rollout. Scanner installation, database checkout, SHA verification and policy errors also fail the job.
