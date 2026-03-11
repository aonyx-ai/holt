<!-- markdownlint-disable-file MD024 -->

# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog], and this project adheres to [Semantic
Versioning].

## [Unreleased]

## [0.2.1] - 2026-03-11

### Fixed

- Include pre-compiled CSS in published holt-book crate (#489)
- Publish holt-regression to crates.io so `cargo install holt-cli` works (#488)

## [0.2.0] - 2026-03-11

### Added

- Add Dockerfile for kit-docs storybook (#448)
- Add holt-regression crate for visual regression testing (#457)
- Add source-level story scanner to holt-regression (#458)
- Add --headless/--save/--check flags to `holt snapshot` (#460)
- Add example crate and fix snapshot Caddy config (#463)
- Add release infrastructure (#469)
- Add Renovate management for Flox version and pre-commit hooks (#481)
- Ship pre-compiled CSS from component libraries (#483)
- Add end-to-end browser tests for kit-docs storybook (#485)

### Changed

- Remove storybook top bar, add mobile-only header (#400)
- Migrate tests from wasm-bindgen-test to plain #[test] (#449)
- Refactor CLI snapshot to use trunk + caddy via doco (#459)

### Fixed

- Fix mobile sidebar SSR + scroll containment (#401)
- Fix test-rust recipe and holt-macros crate ambiguity (#455)
- Fix visual regression check blocking unrelated PRs (#466)
- Skip invalid feature combos in check-features (#470)
- Fix inaccurate documentation across docs/ (#472, #486)
- Add Tailwind CSS styling to basic example (#478)

## [0.1.0] - 2026-02-16

Initial release of Holt: a UI toolkit for Leptos implementing Shadcn/Radix-style
components with behavior/presentation separation.

**Components:** Badge, Breadcrumb, Button, Card, Checkbox, Collapsible, Input,
Label, Select (with keyboard navigation and focus management), Separator,
Switch, Textarea, Toggle, and Typography.

**Storybook framework (holt-book):** Inventory-based story discovery, variant
source code extraction and display, rustdoc-powered documentation, routing,
sidebar navigation, and SSG support with hydration.

**CLI (holt-cli):** `holt serve` for development with hot reloading,
`holt build` for static builds, and `holt snapshot` for visual regression
testing with headless Firefox via geckodriver.

**Infrastructure:** Flox-based dev environment, visual regression CI with
baseline comparison and PR comments, Docusaurus documentation site, and
automated crates.io publishing workflow.

[keep a changelog]: https://keepachangelog.com/en/1.0.0/
[semantic versioning]: https://semver.org/spec/v2.0.0.html
[unreleased]: https://github.com/aonyx-ai/holt/compare/v0.2.1...HEAD
[0.2.1]: https://github.com/aonyx-ai/holt/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/aonyx-ai/holt/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/aonyx-ai/holt/commits/v0.1.0
