//! Leptos web frontend — app shell and route registration.
//!
//! This module contains the root `<UlyssesShell>` component, analogous
//! to `web/src/routes/__root.tsx` in the original React app. It sets
//! up the HTML document, meta tags, global stylesheet, and the router
//! with a shared header/nav layout.

pub mod domain;
pub mod routes;
pub mod server_fns;

use leptos::config::LeptosOptions;
use leptos::prelude::*;
use leptos_meta::{MetaTags, Title, Stylesheet, provide_meta_context};
use leptos_router::components::{Route, Router, Routes};
use leptos_router::StaticSegment;

/// Root Leptos component — the shell that wraps all routes.
///
/// Renders the complete HTML document with `<head>` and `<body>`,
/// following the standard Leptos starter template pattern.
///
/// `leptos_options` is used by `<AutoReload>` and `<HydrationScripts>`
/// for hot-reload and WASM hydration during development.
#[component]
#[allow(clippy::needless_pass_by_value)]
pub fn UlyssesShell(
    /// Leptos configuration options (output name, site root, etc.)
    leptos_options: LeptosOptions,
) -> impl IntoView {
    provide_meta_context();

    view! {
        <!DOCTYPE html>
        <html lang="en" dir="ltr">
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1.0" />
                <meta
                    name="description"
                    content="Proxy and model manager for self-hosted LLMs"
                />
                <AutoReload options=leptos_options.clone() />
                <HydrationScripts options=leptos_options.clone() />
                <MetaTags />
                <Title text="Ulysses" />
                // Global styles (Tailwind v4 CSS)
                <Stylesheet id="leptos" href="/pkg/ulysses.css" />
            </head>
            <body>
                <App />
            </body>
        </html>
    }
}

/// App content — router with layout, navigation, and pages.
#[component]
pub fn App() -> impl IntoView {
    view! {
        <div class="min-h-screen flex flex-col bg-background text-foreground">
            <header class="border-b border-border">
                <nav class="container mx-auto flex items-center justify-between px-4 py-3">
                    <div class="flex items-center gap-6">
                        <a href="/" class="text-lg font-bold tracking-tight">
                            "Ulysses"
                        </a>
                        <div class="flex items-center gap-4 text-sm">
                            <a
                                href="/dashboard"
                                class="text-muted-foreground hover:text-foreground transition-colors"
                            >
                                "Dashboard"
                            </a>
                            <a
                                href="/settings"
                                class="text-muted-foreground hover:text-foreground transition-colors"
                            >
                                "Settings"
                            </a>
                        </div>
                    </div>
                </nav>
            </header>
            <main class="flex-1 container mx-auto px-4 py-6">
                <Router>
                    <Routes fallback=|| view! { <NotFoundPage /> }>
                        <Route
                            path=StaticSegment("")
                            view=|| view! { <routes::home::HomePage/> }
                        />
                        <Route
                            path=StaticSegment("dashboard")
                            view=|| view! { <routes::dashboard::DashboardPage/> }
                        />
                        <Route
                            path=StaticSegment("settings")
                            view=|| view! { <routes::settings::SettingsPage/> }
                        />
                    </Routes>
                </Router>
            </main>
        </div>
    }
}

/// 404 fallback page.
#[component]
fn NotFoundPage() -> impl IntoView {
    view! {
        <div class="flex flex-col items-center justify-center py-20 text-center">
            <h1 class="text-4xl font-bold">"404"</h1>
            <p class="text-muted-foreground mt-2">"Page not found"</p>
            <a href="/" class="mt-4 text-primary hover:underline">
                "Go home"
            </a>
        </div>
    }
}
