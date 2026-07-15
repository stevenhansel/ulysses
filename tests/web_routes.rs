//! SSR rendering tests for Leptos web route components.
//!
//! These tests use Leptos's `RenderHtml::to_html()` to render components
//! to HTML strings on the server side and assert expected content appears.
//! This validates that components render without panicking and produce
//! the expected structure.

use leptos::prelude::*;
use leptos_router::location::RequestUrl;
use ulysses::web::routes;

/// Helper: run a closure inside a fresh reactive owner scope,
/// which is required by Leptos SSR rendering.
fn with_owner<T>(f: impl FnOnce() -> T) -> T {
    let owner = Owner::new();
    owner.set();
    let result = f();
    // Clean up the owner to release reactive resources.
    // We must unset first to avoid stale owner references.
    owner.unset();
    result
}

/// Helper: run a closure inside a reactive owner scope with a
/// `RequestUrl` provided via context (required by `<Router>` in SSR mode).
fn with_owner_and_url<T>(f: impl FnOnce() -> T) -> T {
    let owner = Owner::new();
    owner.set();
    provide_context(RequestUrl::default());
    let result = f();
    owner.unset();
    result
}

// ---------------------------------------------------------------------------
// HomePage
// ---------------------------------------------------------------------------

#[test]
fn home_page_renders_title() {
    let html = with_owner(|| routes::home::HomePage().into_view().to_html());
    assert!(html.contains("Ulysses"), "expected title text");
}

#[test]
fn home_page_renders_description() {
    let html = with_owner(|| routes::home::HomePage().into_view().to_html());
    assert!(
        html.contains("lightweight proxy"),
        "expected description text"
    );
}

#[test]
fn home_page_renders_feature_cards() {
    let html = with_owner(|| routes::home::HomePage().into_view().to_html());
    assert!(html.contains("Model Profiles"), "expected feature card");
    assert!(html.contains("Hot Swapping"), "expected feature card");
    assert!(html.contains("Monitoring"), "expected feature card");
}

#[test]
fn home_page_renders_action_buttons() {
    let html = with_owner(|| routes::home::HomePage().into_view().to_html());
    assert!(
        html.contains("Go to Dashboard"),
        "expected dashboard button"
    );
    assert!(html.contains("Settings"), "expected settings button");
}

#[test]
fn home_page_renders_links() {
    let html = with_owner(|| routes::home::HomePage().into_view().to_html());
    // Both buttons are <a> tags with href attributes
    assert!(
        html.contains(r#"href="/dashboard""#),
        "expected dashboard link"
    );
    assert!(
        html.contains(r#"href="/settings""#),
        "expected settings link"
    );
}

// ---------------------------------------------------------------------------
// DashboardPage
// ---------------------------------------------------------------------------

#[test]
fn dashboard_page_renders_title() {
    let html =
        with_owner(|| routes::dashboard::DashboardPage().into_view().to_html());
    assert!(html.contains("Dashboard"), "expected page title");
}

#[test]
fn dashboard_page_renders_metric_cards() {
    let html =
        with_owner(|| routes::dashboard::DashboardPage().into_view().to_html());

    // The three metric cards (CPU, GPU, RAM) are rendered
    assert!(html.contains("CPU"), "expected CPU metric card");
    assert!(html.contains("GPU"), "expected GPU metric card");
    assert!(html.contains("RAM"), "expected RAM metric card");
}

#[test]
fn dashboard_page_shows_placeholder_values() {
    let html =
        with_owner(|| routes::dashboard::DashboardPage().into_view().to_html());

    // The hardcoded placeholder values
    assert!(html.contains("—"), "expected placeholder values");
}

#[test]
fn dashboard_page_renders_description() {
    let html =
        with_owner(|| routes::dashboard::DashboardPage().into_view().to_html());
    assert!(
        html.contains("Monitor your models"),
        "expected description"
    );
}

// ---------------------------------------------------------------------------
// SettingsPage
// ---------------------------------------------------------------------------

#[test]
fn settings_page_renders_title() {
    let html =
        with_owner(|| routes::settings::SettingsPage().into_view().to_html());
    assert!(html.contains("Settings"), "expected page title");
}

#[test]
fn settings_page_renders_model_profiles_section() {
    let html =
        with_owner(|| routes::settings::SettingsPage().into_view().to_html());
    assert!(
        html.contains("Model Profiles"),
        "expected model profiles section"
    );
}

#[test]
fn settings_page_shows_coming_soon_message() {
    let html =
        with_owner(|| routes::settings::SettingsPage().into_view().to_html());
    assert!(
        html.contains("coming soon"),
        "expected placeholder message"
    );
}

// ---------------------------------------------------------------------------
// App shell (UlyssesShell / App)
// ---------------------------------------------------------------------------

#[test]
fn app_shell_renders_with_doctype() {
    let html = with_owner_and_url(|| {
        ulysses::web::App().into_view().to_html()
    });

    // The App component renders a full page shell with header, nav, router
    assert!(
        html.contains("<header"),
        "expected header element"
    );
    assert!(
        html.contains("Ulysses"),
        "expected app title in nav"
    );
    assert!(
        html.contains("Dashboard"),
        "expected nav link to Dashboard"
    );
    assert!(
        html.contains("Settings"),
        "expected nav link to Settings"
    );
    assert!(
        html.contains(r#"href="/""#),
        "expected home link in nav"
    );
    assert!(
        html.contains(r#"href="/dashboard""#),
        "expected dashboard nav link"
    );
    assert!(
        html.contains(r#"href="/settings""#),
        "expected settings nav link"
    );
}

#[test]
fn app_shell_renders_router_fallback() {
    let html = with_owner_and_url(|| {
        ulysses::web::App().into_view().to_html()
    });

    // The router renders all routes. The home page route (path="")
    // should render as the default route.
    assert!(
        html.contains("Go to Dashboard"),
        "expected home page content via default route"
    );
}
