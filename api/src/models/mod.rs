//! Database models shared across modules.
//!
//! Each model is a plain Rust struct deriving `sqlx::FromRow` and `serde::Serialize`/`Deserialize`.
//! Models with `#[derive(TS)]` get auto-exported to TypeScript via `ts-rs`.
