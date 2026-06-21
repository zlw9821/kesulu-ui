use leptos::prelude::*;

/// Shared selection state for a `RadioGroup`, consumed by each `RadioGroupItem`
/// via context (the project's Radix-context analogue — see CONVENTIONS rule 5).
#[derive(Clone, Copy)]
struct RadioGroupCtx {
    value: Signal<String>,
    on_change: Callback<String>,
}

#[component]
pub fn RadioGroup(
    #[prop(into)] value: Signal<String>,
    #[prop(into)] on_change: Callback<String>,
    #[prop(optional, into)] class: String,
    children: Children,
) -> impl IntoView {
    provide_context(RadioGroupCtx { value, on_change });
    view! {
        <div role="radiogroup" class=format!("grid gap-2 {}", class)>
            {children()}
        </div>
    }
}

#[component]
pub fn RadioGroupItem(
    #[prop(into)] value: String,
    #[prop(optional, into)] label: String,
    #[prop(optional, into)] disabled: MaybeProp<bool>,
    #[prop(optional, into)] class: String,
) -> impl IntoView {
    let ctx = expect_context::<RadioGroupCtx>();
    let selected = {
        let v = value.clone();
        Memo::new(move |_| ctx.value.get() == v)
    };
    let is_disabled = move || disabled.get().unwrap_or(false);
    let click_value = value;
    view! {
        <label class=format!("inline-flex items-center gap-2 cursor-pointer select-none {}", class)>
            <button
                type="button"
                role="radio"
                aria-checked=move || selected.get().to_string()
                disabled=is_disabled
                on:click=move |_| {
                    if !is_disabled() {
                        ctx.on_change.run(click_value.clone());
                    }
                }
                class=move || {
                    format!(
                        "aspect-square h-4 w-4 shrink-0 rounded-full border border-input flex items-center justify-center ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50 {}",
                        if selected.get() { "border-primary" } else { "" },
                    )
                }
            >
                <Show when=move || selected.get()>
                    <span class="h-2 w-2 rounded-full bg-primary"></span>
                </Show>
            </button>
            {(!label.is_empty())
                .then(|| {
                    view! { <span class="text-sm font-medium leading-none">{label}</span> }
                })}
        </label>
    }
}
