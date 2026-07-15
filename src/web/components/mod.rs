//! Reusable UI components.
//!
//! Three tiers:
//! - `ui/` — Base primitives from rust-ui registry (`ui add <component>`)
//! - `layout/` — Structural layout (shell, header, nav, footer)
//! - (higher-level) — Domain-specific in `src/web/domain/`

pub mod layout;
pub mod ui;
