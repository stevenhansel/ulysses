//! Settings page component.
//!
//! Mirrors `web/src/routes/settings.tsx` from the original React app.

use leptos::prelude::*;

/// Settings page — manage model profiles and application preferences.
#[component]
pub fn SettingsPage() -> impl IntoView {
    view! {
        <div class="py-6">
            <h1 class="text-2xl font-bold tracking-tight">"Settings"</h1>
            <p class="text-muted-foreground mt-1">
                "Manage your model profiles and application preferences."
            </p>
            <div class="mt-8 max-w-xl space-y-8">
                <section class="rounded-lg border border-border p-6">
                    <h2 class="text-lg font-semibold">"Model Profiles"</h2>
                    <p class="text-sm text-muted-foreground mt-1">
                        "Configure inference backends and model identifiers."
                    </p>
                    <p class="text-sm text-muted-foreground mt-4 italic">
                        "Profile management coming soon."
                    </p>
                </section>
            </div>
        </div>
    }
}
