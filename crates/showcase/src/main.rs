//! CSR entry for the kesulu-ui showcase. Mounts the living gallery to <body>.
//! Run with `trunk serve` (see this crate's README/Trunk.toml).
mod showcase;

use leptos::prelude::*;
use showcase::UiShowcasePage;

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(|| {
        view! {
            <div class="min-h-screen p-6 md:p-10">
                <UiShowcasePage />
            </div>
        }
    });
}
