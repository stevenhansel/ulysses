//! Dashboard page component.
//!
//! Mirrors `web/src/routes/dashboard.tsx` from the original React app.

use leptos::prelude::*;

/// Dashboard page — shows hardware metrics and model status.
#[component]
pub fn DashboardPage() -> impl IntoView {
    view! {
        <div class="py-6">
            <h1 class="text-2xl font-bold tracking-tight">"Dashboard"</h1>
            <p class="text-muted-foreground mt-1">
                "Monitor your models, hardware, and inference performance."
            </p>
            <div class="mt-8 grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
                <div class="rounded-lg border border-border p-6">
                    <h2 class="text-sm font-medium text-muted-foreground">"CPU"</h2>
                    <p class="text-2xl font-bold mt-2">"—"</p>
                </div>
                <div class="rounded-lg border border-border p-6">
                    <h2 class="text-sm font-medium text-muted-foreground">"GPU"</h2>
                    <p class="text-2xl font-bold mt-2">"—"</p>
                </div>
                <div class="rounded-lg border border-border p-6">
                    <h2 class="text-sm font-medium text-muted-foreground">"RAM"</h2>
                    <p class="text-2xl font-bold mt-2">"—"</p>
                </div>
            </div>
        </div>
    }
}
