// Shared types (available in both SSR and WASM builds)
pub mod types;

// Web frontend (available in both builds)
pub mod web;

// Server-only modules (gated behind `ssr` feature)
#[cfg(feature = "ssr")]
pub mod api;
#[cfg(feature = "ssr")]
pub mod config;
#[cfg(feature = "ssr")]
pub mod context;
#[cfg(feature = "ssr")]
pub mod error;
#[cfg(feature = "ssr")]
pub mod models;
