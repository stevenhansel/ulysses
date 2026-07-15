//! Profile card component — displays a single model configuration profile.
//!
//! Mirrors `web/src/features/profiles/components/profile-card.tsx`.

use icons::common::IconType;
use icons::leptos::icon_component::LeptosIcon;
use leptos::prelude::*;

use crate::types::ProfileResponse;

/// Displays a single proxy profile in a card.
#[allow(clippy::needless_pass_by_value)]
#[component]
pub fn ProfileCard(
    profile: ProfileResponse,
    #[prop(optional)]
    on_edit: Option<Callback<ProfileResponse>>,
    #[prop(optional)]
    on_delete: Option<Callback<ProfileResponse>>,
) -> impl IntoView {
    view! {
        <div class="rounded-lg border bg-card text-card-foreground shadow-sm">
            <div class="flex flex-row items-start justify-between p-6 pb-3">
                <div>
                    <h3 class="text-base font-semibold leading-none">
                        {profile.name.clone()}
                    </h3>
                    <p class="mt-1 text-sm text-muted-foreground">
                        {profile.backend_url.clone()}
                    </p>
                </div>
                <div class="flex items-center gap-1">
                    {on_edit.map(|cb| {
                        let profile = profile.clone();
                        view! {
                            <button
                                class="inline-flex items-center justify-center gap-2 whitespace-nowrap rounded-md text-sm font-medium transition-colors hover:bg-accent hover:text-accent-foreground size-9"
                                aria-label="Edit profile"
                                on:click=move |_| { cb.run(profile.clone()); }
                            >
                                <LeptosIcon icon=IconType::Pencil class="h-4 w-4" />
                            </button>
                        }
                    })}
                    {on_delete.map(|cb| {
                        let profile = profile.clone();
                        view! {
                            <button
                                class="inline-flex items-center justify-center gap-2 whitespace-nowrap rounded-md text-sm font-medium transition-colors hover:bg-accent hover:text-accent-foreground size-9"
                                aria-label="Delete profile"
                                on:click=move |_| { cb.run(profile.clone()); }
                            >
                                <LeptosIcon icon=IconType::Trash2 class="h-4 w-4" />
                            </button>
                        }
                    })}
                </div>
            </div>
            <div class="px-6 pb-6">
                <div class="grid grid-cols-2 gap-4 text-sm">
                    <div>
                        <span class="text-muted-foreground">"Status"</span>
                        <p class="font-medium">
                            {if profile.active { "Active" } else { "Inactive" }}
                        </p>
                    </div>
                    <div>
                        <span class="text-muted-foreground">"ID"</span>
                        <p class="font-medium truncate" title=profile.id.clone()>
                            {profile.id.clone()}
                        </p>
                    </div>
                </div>
            </div>
        </div>
    }
}
