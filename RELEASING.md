# Releasing

This repository uses a staged multi-crate release flow rather than a single
crate manual publish path.

## Current release state

`use-optimization` publishes independent focused crates first, then the focused
crates that depend on `use-objective`, and finally the `use-optimization`
facade crate.

## Canonical release guide

Use [RELEASE.md](RELEASE.md) as the authoritative release policy for:

- first publish wave scope and ordering
- staged publish-readiness checks
- trusted publishing setup after the initial manual wave
- maintainer release sequencing

## Current automation

The repository includes the release workflows that match this staged shape:

- `publish-readiness.yml`
- `dependent-publish-readiness.yml`
- `facade-publish-readiness.yml`
- `release-plz-pr.yml`
- `release-plz-release.yml`
