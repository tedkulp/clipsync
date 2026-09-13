---
name: clipsync-release
description: Use when releasing a new version of ClipSync — bumping versions across Cargo/Tauri/npm/PKGBUILD, updating the changelog, tagging, publishing the draft GitHub Release, and updating the Homebrew cask.
---

# ClipSync Release

## Overview

Releases are triggered by pushing a `v*` tag. Two workflows run on the tag:

- **Build Desktop Apps** (`build-desktop.yml`) — builds macOS universal DMG, Linux deb/rpm, Windows exe/msi, then `create-release` makes a **draft** GitHub Release with those files.
- **Build Server** (`build-server.yml`) — tests, builds the server binary and multi-arch Docker image, then `release-docker` tags `ghcr.io/tedkulp/clipsync-server:<version>` as `latest`.

The draft must be published by hand, and the Homebrew cask is updated after that. Your job is to prepare the repo, tag the release commit, and walk those follow-up steps.

## Release Process

### Step 1: Preflight

```bash
git status                       # must be clean
git switch main && git pull
gh run list --branch main --limit 3   # CI, Build Server, Build Desktop Apps should be green
```

Don't release on top of a red `main`.

### Step 2: Determine the version

```bash
git tag --sort=-version:refname | head -5
```

- **Patch** (`0.1.1 → 0.1.2`): bug fixes only
- **Minor** (`0.1.x → 0.2.0`): new features, backwards-compatible
- **Major** (`0.x.x → 1.0.0`): breaking changes (e.g. protocol changes that need server and desktop upgraded together)

Read the `## [Unreleased]` section of `CHANGELOG.md` and `git log <last-tag>..HEAD --oneline` to decide.

### Step 3: Bump the version everywhere

The version lives in several places and they must all match:

| File | Field |
|------|-------|
| `Cargo.toml` | `[workspace.package] version` (crates use `version.workspace = true`) |
| `crates/desktop/tauri.conf.json` | `"version"` — drives bundle filenames like `ClipSync_X.Y.Z_universal.dmg` |
| `desktop/package.json` | `"version"` |
| `PKGBUILD` | `pkgver=X.Y.Z` and reset `pkgrel=1` |
| `.SRCINFO` | `pkgver = X.Y.Z`, `pkgrel = 1`, and the version in the `source =` URL (keep in sync with `PKGBUILD`) |

Then refresh the lock files so they match:

```bash
cargo update --workspace                                  # updates clipsync-* entries in Cargo.lock
npm --prefix desktop install --package-lock-only          # updates desktop/package-lock.json
```

Verify nothing was missed (every line should show the new version; `pkgrel` should be 1):

```bash
grep -n '^version' Cargo.toml
grep -n '"version"' crates/desktop/tauri.conf.json
grep -n -m2 '"version"' desktop/package.json desktop/package-lock.json
grep -n -A1 'name = "clipsync' Cargo.lock | grep version
grep -n '^pkgver\|^pkgrel' PKGBUILD
grep -n 'pkgver\|pkgrel\|source' .SRCINFO
```

Don't touch `PKGBUILD.git` — its `pkgver()` is computed from git. Don't touch `clipsync.rb` yet — it's updated in Step 9 once the DMG exists.

### Step 4: Update CHANGELOG.md

The file uses [Keep a Changelog](https://keepachangelog.com) format.

1. Move the entries under `## [Unreleased]` into a new versioned section **directly below** it, with today's date:

```markdown
## [Unreleased]

## [0.1.2] - 2026-09-13

### Added
- ...

### Fixed
- ...
```

2. Leave `## [Unreleased]` in place, empty
3. Sections to use: `Added`, `Changed`, `Deprecated`, `Removed`, `Fixed`, `Security`
4. Fill gaps from `git log <last-tag>..HEAD` — user-visible changes only, not CI-internal noise

### Step 5: Build check

```bash
cargo fmt --all -- --check
cargo clippy --workspace --exclude clipsync-desktop --all-targets -- -D warnings
cargo test --workspace --exclude clipsync-desktop
```

These match the checks in `ci.yml` and `build-server.yml`; a failure here would fail the tag build too.

### Step 6: Commit

```bash
git add Cargo.toml Cargo.lock crates/desktop/tauri.conf.json desktop/package.json desktop/package-lock.json PKGBUILD .SRCINFO CHANGELOG.md
git commit -m "Release vX.Y.Z"
```

Commit message format: `Release vX.Y.Z` — matches existing history (`Release v0.1.1`).

### Step 7: Tag the release commit and push

```bash
git tag vX.Y.Z
git push origin main && git push origin vX.Y.Z
```

Tag **the release commit itself**, immediately after committing. (v0.1.1 ended up on a later fix commit — avoid that.)

Pushing the tag starts Build Desktop Apps and Build Server for the tag. Pushing `main` also starts CI.

### Step 8: Watch the tag builds and publish the draft

```bash
gh run list --branch vX.Y.Z --limit 5
gh run watch <run-id> --exit-status
```

The desktop build takes ~15–20 minutes. When it's done, check:

- `create-release` and `release-docker` jobs **ran** (not skipped) and succeeded
- The draft has all expected assets:

```bash
gh release view vX.Y.Z --json isDraft,assets -q '"draft: \(.isDraft)", (.assets[].name)'
```

Expected: `ClipSync_X.Y.Z_universal.dmg`, `ClipSync_X.Y.Z_amd64.deb`, `ClipSync-X.Y.Z-1.x86_64.rpm`, `ClipSync_X.Y.Z_x64-setup.exe`, `ClipSync_X.Y.Z_x64_en-US.msi`.

- The Docker image is tagged: `docker pull ghcr.io/tedkulp/clipsync-server:X.Y.Z`

Review the auto-generated notes (edit in the changelog wording if helpful), then **ask the user before publishing** — publishing is public:

```bash
gh release edit vX.Y.Z --draft=false
```

### Step 9: Update the Homebrew cask

Only after the release is published (draft assets aren't publicly downloadable):

```bash
./scripts/update-homebrew-formula.sh vX.Y.Z
git diff clipsync.rb        # version and sha256 should both be set
git add clipsync.rb
git commit -m "Update Homebrew cask to vX.Y.Z"
git push origin main
```

Then, **with the user's okay**, copy it to the tap repo:

```bash
cp clipsync.rb ../homebrew-tap/Casks/
cd ../homebrew-tap
git add Casks/clipsync.rb
git commit -m "Update clipsync to vX.Y.Z"
git push
```

If `../homebrew-tap` doesn't exist locally, tell the user rather than cloning it somewhere unexpected.

## Quick Reference

| Step | Command |
|------|---------|
| Latest tags | `git tag --sort=-version:refname \| head -5` |
| Refresh lock files | `cargo update --workspace && npm --prefix desktop install --package-lock-only` |
| Commit | `git commit -m "Release vX.Y.Z"` |
| Tag + push | `git tag vX.Y.Z && git push origin main && git push origin vX.Y.Z` |
| Tag builds | `gh run list --branch vX.Y.Z` |
| Check draft | `gh release view vX.Y.Z` |
| Publish | `gh release edit vX.Y.Z --draft=false` |
| Homebrew | `./scripts/update-homebrew-formula.sh vX.Y.Z` |

## Common Mistakes

- **Version mismatch** — missing `.SRCINFO` leaves the AUR package pointing at the old tarball; bumping `Cargo.toml` but not `tauri.conf.json` gives DMGs with the old version in the filename, and the Homebrew script then 404s
- **Stale lock files** — forgetting `cargo update --workspace` leaves `Cargo.lock` on the old version; CI's `npm ci` fails if `package-lock.json` doesn't match `package.json`
- **Forgetting to push the tag** — `git push` alone does not push tags
- **Tagging the wrong commit** — tag right after the release commit, before any other commits land
- **Running the Homebrew script against a draft** — the DMG download fails until the release is published
- **Removing `[Unreleased]`** — always leave it, just empty
- **Wrong date** — use today's actual date in `YYYY-MM-DD` format
