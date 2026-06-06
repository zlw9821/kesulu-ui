use leptos::prelude::*;

/// Single-value, signal-driven slider — a faithful port of the shadcn/ui
/// `Slider` visual (track / range / thumb) backed by a native `<input
/// type=range>` for robust keyboard + pointer interaction. The range input is
/// rendered transparently on top of the visual so the styled thumb tracks the
/// value while the browser handles drag math.
#[component]
pub fn Slider(
    #[prop(into)] value: Signal<f64>,
    #[prop(into)] on_change: Callback<f64>,
    #[prop(optional)] min: f64,
    #[prop(optional, default = 100.0)] max: f64,
    #[prop(optional, default = 1.0)] step: f64,
    #[prop(optional, into)] disabled: MaybeProp<bool>,
    #[prop(optional, into)] class: String,
) -> impl IntoView {
    let is_disabled = move || disabled.get().unwrap_or(false);

    // Fill percentage of the range/thumb based on the current value.
    let percent = move || {
        let span = max - min;
        if span <= 0.0 {
            0.0
        } else {
            (((value.get() - min) / span) * 100.0).clamp(0.0, 100.0)
        }
    };

    let root_class = move || {
        format!(
            "relative flex w-full touch-none items-center select-none {} {}",
            if is_disabled() { "opacity-50" } else { "" },
            class,
        )
    };

    view! {
        <span class=root_class>
            // Transparent native range input drives the value + interaction. Declared
            // first so it acts as the Tailwind `peer` for the styled thumb (surfacing a
            // focus ring), while `opacity-0` + the thumb's `pointer-events-none` keep
            // the visual layer on top for interaction.
            <input
                type="range"
                class="peer absolute inset-0 z-10 m-0 h-full w-full cursor-pointer opacity-0 disabled:cursor-not-allowed"
                min=min
                max=max
                step=step
                prop:value=move || value.get().to_string()
                disabled=is_disabled
                on:input=move |ev| {
                    if let Ok(v) = event_target_value(&ev).parse::<f64>() {
                        on_change.run(v);
                    }
                }
            />
            // Track
            <span class="relative grow overflow-hidden rounded-full bg-muted h-1.5 w-full">
                // Range (filled portion)
                <span
                    class="absolute h-full bg-primary left-0 top-0"
                    style=move || format!("width: {}%;", percent())
                ></span>
            </span>
            // Styled thumb, positioned by value. Ring states mirror the native input's
            // hover/focus via the `peer-*` variants.
            <span
                class="pointer-events-none absolute top-1/2 block size-4 shrink-0 -translate-x-1/2 -translate-y-1/2 rounded-full border border-primary bg-background shadow-sm ring-ring/50 transition-[left] peer-hover:ring-4 peer-focus-visible:ring-4 peer-focus-visible:outline-hidden"
                style=move || format!("left: {}%;", percent())
            ></span>
        </span>
    }
}
