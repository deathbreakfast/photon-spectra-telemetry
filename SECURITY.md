# Security Policy

## Supported versions

Security fixes are accepted against the latest `main` branch and tagged releases (`0.1.x` / `0.2.x`) of this repository's crates (`photon-spectra-telemetry`).

## Reporting a vulnerability

Please **do not** open a public GitHub issue for security-sensitive reports.

Prefer one of the following:

1. **GitHub Security Advisories** — use [Report a vulnerability](https://github.com/unified-field-dev/photon-spectra-telemetry/security/advisories/new) on this repository when available.
2. Contact the maintainers privately via the repository owner listed at https://github.com/unified-field-dev/photon-spectra-telemetry.

Include:

- a description of the issue and its impact
- steps to reproduce or a proof of concept when possible
- affected crate names and versions

We will acknowledge receipt as soon as practical and coordinate a fix and disclosure timeline with you.

## Scope

In scope: vulnerabilities in this repository's published crates and documentation that could cause unsafe production defaults, plus CI/supply-chain issues in this repository.

Out of scope: vulnerabilities solely in third-party dependencies unless this project mishandles them in a security-relevant way.

## Telemetry hardening

- `metrics` hashes `topic`, `subscription`, and `topic_key` label values before recording
  counters/gauges to bound cardinality.
- `SpectraOpsLog` forwards only schema-defined metric/event names with allowlisted labels and
  filtered JSON payloads.

## Authentication and authorization

Forwards Photon's operational signals into Spectra. Caller authentication and authorization for Photon are host concerns.

