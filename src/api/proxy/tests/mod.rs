//! Proxy module test suite.
//!
//! Tests are split by layer: integration, service, repository, controllers.
//! Each file tests its corresponding source file.

mod controller_http;
mod controller_ws;
mod integration;
mod repository;
mod service;
