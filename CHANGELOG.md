# Changelog

All notable changes to this project will be documented in this file.

## [0.4.8] - 2026-04-25

### Added

- Add a local macOS packaging script that builds a signed `md-bider.app` bundle and zip archive.

### Changed

- Ad-hoc sign macOS release app bundles in the GitHub release workflow.
- Clarify macOS shortcuts, CLI usage, and app bundle packaging in both READMEs.

## [0.4.6] - 2026-03-06

### Fixed

- Load the embedded editor shell from a custom protocol URL instead of `with_html`, giving the app a stable origin so persisted UI language settings can actually be restored on the next launch.
- Keep the stable WebView data directory wiring introduced in `0.4.5`, so persistence no longer depends on the executable filename or location.

## [0.4.5] - 2026-03-06

### Fixed

- Persist the selected UI language across launches by storing WebView data in a stable application data directory instead of an executable-name-specific WebView2 folder.
- Keep the project build artifact as the canonical local executable so language persistence no longer depends on copying binaries onto the desktop.

### Docs

- Added design and implementation notes for persistent language settings and desktop launcher behavior.

## [0.4.4] - 2026-03-06

### Fixed

- Restore the four split-preview buttons to a real right-side sidebar in `SV` mode.
- Keep the preview container visibility override guarded to `SV` only, so `IR` and `WYSIWYG` still avoid unintended split panes.
- Tightened the regression test to require the split-mode guard around preview layout forcing.

## [0.4.3] - 2026-03-06

### Fixed

- Keep the preview sidebar hidden in `IR` and `WYSIWYG` modes so split layout only appears in `SV`.
- Added a regression test to prevent preview-container visibility overrides from reintroducing extra split panes.

## [0.1.0] - 2026-03-03

### Added

- New file workflow: toolbar `新建` button and `Ctrl + N` shortcut.
- Full offline embedded assets for editor runtime (JS/CSS/icon/i18n).
- Startup file argument support: open a markdown file directly from CLI.

### Changed

- Default editor mode switched to `IR` (所见即所得).
- Toolbar action set restored and hardened for daily authoring.
- Chinese-first typography and display stability improvements.

### Fixed

- Resource bootstrap failures caused by missing injected payloads.
- Missing toolbar icons and language runtime state mismatches.

### Docs

- Rebuilt repository README into a product-style landing page.
- Added contributing guide and clarified development workflow.
