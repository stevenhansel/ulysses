//! Landing page component.
//!
//! Mirrors `web/src/routes/index.tsx` from the original React app.

use icons::common::IconType;
use icons::leptos::icon_component::LeptosIcon;
use leptos::prelude::*;

/// Home / landing page.
#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <div class="flex flex-col items-center justify-center text-center py-20">
            <h1 class="text-4xl font-bold tracking-tight sm:text-5xl">
                "Ulysses"
            </h1>
            <p class="mt-4 text-lg text-muted-foreground max-w-2xl">
                "A lightweight proxy and model manager for self-hosted LLMs. "
                "Route requests, hot-swap models, and monitor performance — all from one dashboard."
            </p>

            <div class="mt-10 grid grid-cols-1 sm:grid-cols-3 gap-6 max-w-2xl w-full">
                <FeatureCard
                    icon=IconType::GitFork
                    title="Model Profiles"
                    description="Configure backends, models, and inference parameters."
                />
                <FeatureCard
                    icon=IconType::Cpu
                    title="Hot Swapping"
                    description="Automatically load and swap models on demand."
                />
                <FeatureCard
                    icon=IconType::Activity
                    title="Monitoring"
                    description="Track hardware and inference metrics in real time."
                />
            </div>

            <div class="mt-10 flex gap-4">
                <a
                    href="/dashboard"
                    class="inline-flex h-10 items-center justify-center rounded-md bg-primary px-6 text-sm font-medium text-primary-foreground shadow transition-colors hover:bg-primary/90"
                >
                    "Go to Dashboard"
                </a>
                <a
                    href="/settings"
                    class="inline-flex h-10 items-center justify-center rounded-md border border-input bg-background px-6 text-sm font-medium shadow-sm transition-colors hover:bg-accent hover:text-accent-foreground"
                >
                    "Settings"
                </a>
            </div>
        </div>
    }
}

/// Reusable feature card for the landing page.
#[component]
fn FeatureCard(
    icon: IconType,
    title: &'static str,
    description: &'static str,
) -> impl IntoView {
    view! {
        <div class="rounded-lg border border-border p-4 text-left">
            <LeptosIcon icon=icon class="h-5 w-5 text-primary mb-2" />
            <h3 class="font-semibold text-sm">{title}</h3>
            <p class="text-xs text-muted-foreground mt-1">{description}</p>
        </div>
    }
}
