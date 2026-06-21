use leptos::prelude::*;
use std::time::Duration;

/// Handle returned by [`use_presence`] — keeps a node mounted through its exit
/// animation. `mounted` stays `true` for `hide_delay_ms` after `when` flips to
/// `false`, so an `animate-out` class can finish before the node unmounts;
/// `open` mirrors `when` (use it to pick enter- vs exit-animation classes).
#[derive(Clone, Copy)]
pub struct Presence {
    pub mounted: Signal<bool>,
    pub open: Signal<bool>,
}

/// Delayed-unmount ("presence") for animated overlays — the Leptos analogue of
/// Radix `Presence` / framer-motion `AnimatePresence`.
///
/// SSR-safe: the timing `Effect` runs client-only, and `mounted` is initialised
/// from `when` so the server and first client render agree. No web-sys — it uses
/// leptos's own `set_timeout`.
///
/// ```ignore
/// let p = use_presence(open.into(), 150);
/// view! {
///     <Show when=move || p.mounted.get()>
///         <div class=move || format!(
///             "<base> {}",
///             if p.open.get() { "animate-in fade-in-0 zoom-in-95" }
///             else { "animate-out fade-out-0 zoom-out-95" },
///         )>...</div>
///     </Show>
/// }
/// ```
pub fn use_presence(when: Signal<bool>, hide_delay_ms: u64) -> Presence {
    let mounted = RwSignal::new(when.get_untracked());
    Effect::new(move |_| {
        if when.get() {
            mounted.set(true);
        } else if mounted.get_untracked() {
            set_timeout(
                move || {
                    // Only unmount if still closed — guards a rapid re-open
                    // during the exit window.
                    if !when.get_untracked() {
                        mounted.set(false);
                    }
                },
                Duration::from_millis(hide_delay_ms),
            );
        }
    });
    Presence {
        mounted: mounted.into(),
        open: when,
    }
}
