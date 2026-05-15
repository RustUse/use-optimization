# Release Policy

RustUse/use-optimization is not published yet. The workspace keeps
`publish = false` at the root package level while the intentional first-wave
crate manifests opt into `publish = true`.

Because the workspace contains internal dependencies between focused crates, the
first public crates.io rollout is staged in three waves instead of one.

## First Publish Waves

### Wave 1: Independent focused crates

Publish these crates first:

- `use-objective`
- `use-optimization-constraint`
- `use-search-space`
- `use-score`
- `use-loss`

These crates do not depend on other publishable crates from the same
workspace.

### Wave 2: Dependent focused crates

After Wave 1 is visible on crates.io, publish:

- `use-grid-search`
- `use-local-search`

Both depend on `use-objective`, so their dry-runs and live publishes must wait
for `use-objective` to resolve from crates.io.

### Wave 3: Facade crate

After every focused crate from Waves 1 and 2 is visible on crates.io, publish:

- `use-optimization`

The facade dry-run is intentionally separate because it depends on the full
focused crate surface being available in the registry.

## Publish Surface

Before the first publish wave, confirm that the release surface:

- keeps the workspace-level default at `publish = false`
- keeps the focused crates at `publish = true`
- keeps `crates/use-optimization/Cargo.toml` at `publish = true`
- leaves any future non-release crates opted out until intentionally reviewed

## Versioning

- The workspace uses lockstep `0.x.y` versioning.
- Before `1.0`, breaking changes should bump the minor version.
- Before `1.0`, additive compatible changes should bump the patch version.
- The facade crate should only advertise the focused crates that are actively
  supported.

## Automated Release Validation

The repository includes a staged release-validation path:

- `.github/workflows/publish-readiness.yml` runs on pull requests, pushes to
  `main`, and manual dispatch for Wave 1 crates.
- `.github/workflows/dependent-publish-readiness.yml` is a manual Wave 2 gate
  for `use-grid-search` and `use-local-search` after Wave 1 crates resolve from
  crates.io.
- `.github/workflows/facade-publish-readiness.yml` is the final manual facade
  check after all focused crates are live on crates.io.
- `.github/workflows/release-plz-pr.yml` prepares lockstep version bumps and
  changelog updates.
- `.github/workflows/release-plz-release.yml` remains post-initial-release
  automation only and is guarded behind `CRATES_IO_AUTOPUBLISH_ENABLED=true` or
  manual dispatch.

## One-time post-initial-release setup

Before relying on automated publishing:

- Configure crates.io Trusted Publishing for every published crate with
  repository owner `RustUse`, repository name `use-optimization`, and workflow
  filename `release-plz-release.yml`.
- Leave the crates.io environment field empty unless you intentionally add a
  matching GitHub Actions environment later.
- Set the repository variable `CRATES_IO_AUTOPUBLISH_ENABLED` to `true` only
  after the initial manual crates.io wave is complete.
- Do not set `CARGO_REGISTRY_TOKEN` for the release-plz workflow when using
  trusted publishing.

## Maintainer Release Checklist

For the initial public release:

1. Run the full quality checks and Wave 1 publish-readiness flow.
2. Publish Wave 1 crates manually.
3. Wait for crates.io index propagation.
4. Run the Wave 2 publish-readiness flow and publish `use-grid-search` and
   `use-local-search`.
5. Wait for crates.io index propagation again.
6. Run the facade publish-readiness flow.
7. Publish `use-optimization`.

For normal post-initial-release releases:

1. Merge ordinary PRs with clean conventional commit style in the final commit
   subject or squash title.
2. Let `Release PR Automation` open or update the release PR.
3. Review the lockstep version bump and generated `CHANGELOG.md`.
4. Merge the release PR.
5. Let `Release Publish Automation` publish from the merged release commit, or
   manually dispatch it with `post-initial-release = true` if needed.

## Publish Readiness Checklist

1. Confirm `cargo fmt --all -- --check` is clean.
2. Confirm `cargo clippy --workspace --all-targets -- -D warnings` passes.
3. Confirm `cargo test --workspace` passes.
4. Confirm `cargo doc --workspace --no-deps` passes.
5. Confirm Wave 1 dry-runs pass before the first publish attempt.
6. Confirm Wave 2 dry-runs pass after `use-objective` is visible on crates.io.
7. Confirm the facade dry-run passes only after all focused crates are visible
   on crates.io.
