# Release a new version of tdo

Cuts a new version of tdo: bumps `Cargo.toml`, commits, tags, pushes — which fires the GitHub Actions release pipeline that builds binaries and updates the Homebrew formula.

## Instructions

1. Read the current version from `Cargo.toml` (the `version = "x.y.z"` line under `[package]`).

2. Compute the default next version by bumping the patch component (e.g. `0.1.1` → `0.1.2`).

3. Ask the user which version to release, presenting the current version and three options. Use the AskUserQuestion tool with:
   - Question: `Release which version? (current: <CURRENT>)`
   - Options:
     - `patch` — bump patch (e.g. `<CURRENT>` → `<NEXT_PATCH>`) — for bug fixes
     - `minor` — bump minor (e.g. `<CURRENT>` → `<NEXT_MINOR>`) — for new features
     - `major` — bump major (e.g. `<CURRENT>` → `<NEXT_MAJOR>`) — for breaking changes
   - Allow other answer: yes (so the user can type an exact version like `0.2.0-rc.1`)

4. Resolve the chosen bump to a concrete version string:
   - `patch` → bump the third component
   - `minor` → bump the second component, reset patch to 0
   - `major` → bump the first component, reset minor and patch to 0
   - Free-form answer → use it verbatim (strip a leading `v` if present)

5. Run `make release VERSION=<resolved-version>` from the repo root.

6. After the make target completes, tell the user:
   - The new version was tagged and pushed
   - Link to the running release workflow: `https://github.com/Wildhoney/Tdo/actions`
   - Once the workflow finishes, they can `brew update && brew upgrade tdo`

## Notes

- The makefile's `release` target enforces a clean working tree and `main` branch — if it fails preflight, surface the message and stop. Don't try to commit on the user's behalf.
- Don't bump the version manually with Edit — `make release` owns that step. Your job is to pick the version and invoke make.
- If `make release` fails partway, the makefile reverts `Cargo.toml`/`Cargo.lock` automatically. No manual cleanup needed.
