# Repository Guidelines

## Project Structure & Module Organization
- `src/` holds the Dioxus components (`button.rs`, `card.rs`, `input.rs`, `label.rs`), shared styling (`theme.rs`), utilities, and a small `prelude` for common exports. `lib.rs` re-exports the public surface.
- `assets/tailwind.css` is generated; `tailwind.css` is the source template. `build.rs` downloads the Tailwind CLI if absent and regenerates the compiled CSS during `cargo build`.
- `demo/` is a companion app showing the components in use (`cargo run -p demo`). Treat it as the quickest way to validate changes.
- `ui/` contains TypeScript/React versions of some components for parity reference only; it is not built by Cargo.

## Build, Test, and Development Commands
- `cargo build` — compiles the library and runs the Tailwind build script to refresh `assets/tailwind.css`.
- `cargo test` — runs Rust unit/integration tests. Add focused component tests before new exports land.
- `cargo run -p demo` — launches the demo app showcasing the components.
- `cargo fmt` then `cargo clippy --all-targets --all-features` — format and lint before submitting. Fix warnings or annotate with clear justification.

## Coding Style & Naming Conventions
- Rust 2024 edition; rely on `cargo fmt` defaults. Keep lines readable and favor early returns over deep nesting.
- Module and file names are `snake_case`; exported components use PascalCase functions annotated with `#[component]` (e.g., `Button`), while variants/sizes use enums with clear, descriptive variants.
- Keep styling tokens in `const` strings to aid reuse and minimize typos. Prefer `SmolStr` for borrowed/optional class inputs.
- When adding components, mirror the existing pattern: props with `#[props(default)]` where sensible and explicit `data-slot` hooks for consumers.

## Testing Guidelines
- Co-locate unit tests in `#[cfg(test)]` modules near the component code, or use `tests/` for integration. Name tests for behavior (`button_handles_clicks`, `input_applies_custom_class`).
- Validate emitted class lists, prop defaults, and event handling. Add demo coverage when the behavior is visual or interaction-heavy.
- Run `cargo test` locally before pushing; add screenshots from the demo for visual changes.

## Commit & Pull Request Guidelines
- Follow the existing short, imperative commit style (`add button component`, `add more components`). One logical change per commit keeps history clean.
- Pull requests should include: a brief summary of changes, linked issues (if any), commands/tests executed, and screenshots/gifs for UI-affecting work (demo output is fine).
- Ensure `assets/tailwind.css` stays in sync with `tailwind.css` after styling changes; rerun `cargo build` if Tailwind inputs change. Note any build-script networking needs when applicable.
