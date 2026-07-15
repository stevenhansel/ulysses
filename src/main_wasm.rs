//! WASM entry point for Leptos hydration.
//!
//! This binary is compiled only for `wasm32-unknown-unknown` and
//! hydrates the SSR-rendered HTML in the browser with interactive
//! components. The actual component tree is imported from the
//! shared `web` module.

#[cfg(target_arch = "wasm32")]
fn main() {
    use leptos::config::LeptosOptions;
    use leptos::prelude::*;
    use ulysses::web::UlyssesShell;

    console_error_panic_hook::set_once();

    // Build default LeptosOptions for the hydrate entry point.
    // In production, these would match the server's options.
    let leptos_options = LeptosOptions::builder()
        .output_name("ulysses")
        .site_pkg_dir("pkg")
        .site_root("site")
        .build();

    mount_to_body(|| view! { <UlyssesShell leptos_options /> });
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    eprintln!("This binary is only meant for the wasm32 target.");
}
