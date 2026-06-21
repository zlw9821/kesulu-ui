use leptos::prelude::*;

#[derive(Default, Clone, Copy, PartialEq)]
pub enum ToggleVariant {
    #[default]
    Default,
    Outline,
}

impl ToggleVariant {
    fn classes(self) -> &'static str {
        match self {
            ToggleVariant::Default => "bg-transparent",
            ToggleVariant::Outline => {
                "border border-input bg-transparent shadow-xs hover:bg-accent hover:text-accent-foreground"
            }
        }
    }
}

#[derive(Default, Clone, Copy, PartialEq)]
pub enum ToggleSize {
    Sm,
    #[default]
    Default,
    Lg,
}

impl ToggleSize {
    fn classes(self) -> &'static str {
        match self {
            ToggleSize::Sm => "h-8 min-w-8 px-1.5",
            ToggleSize::Default => "h-9 min-w-9 px-2",
            ToggleSize::Lg => "h-10 min-w-10 px-2.5",
        }
    }
}

/// Base classes shared by `Toggle` and `ToggleGroupItem` (the `toggleVariants`
/// analogue). `data-[state=on]` becomes a signal-driven `on_class` appended below.
const TOGGLE_BASE: &str = "inline-flex items-center justify-center gap-2 rounded-md text-sm font-medium whitespace-nowrap transition-[color,box-shadow] outline-none hover:bg-muted hover:text-muted-foreground focus-visible:border-ring focus-visible:ring-[3px] focus-visible:ring-ring/50 disabled:pointer-events-none disabled:opacity-50 [&_svg]:pointer-events-none [&_svg]:shrink-0 [&_svg:not([class*='size-'])]:size-4";

#[component]
pub fn Toggle(
    #[prop(into)] pressed: Signal<bool>,
    #[prop(into)] on_change: Callback<bool>,
    #[prop(optional)] variant: ToggleVariant,
    #[prop(optional)] size: ToggleSize,
    #[prop(optional, into)] disabled: MaybeProp<bool>,
    #[prop(optional, into)] class: String,
    children: Children,
) -> impl IntoView {
    let variant_class = variant.classes();
    let size_class = size.classes();
    let is_disabled = move || disabled.get().unwrap_or(false);
    view! {
        <button
            type="button"
            aria-pressed=move || pressed.get().to_string()
            data-state=move || if pressed.get() { "on" } else { "off" }
            disabled=is_disabled
            on:click=move |_| {
                if !is_disabled() {
                    on_change.run(!pressed.get());
                }
            }
            class=move || {
                let on_class = if pressed.get() { "bg-accent text-accent-foreground" } else { "" };
                format!("{TOGGLE_BASE} {variant_class} {size_class} {on_class} {class}")
            }
        >
            {children()}
        </button>
    }
}

/// Shared selection state + styling for a `ToggleGroup`, consumed by each
/// `ToggleGroupItem` via context (the project's Radix-context analogue).
#[derive(Clone, Copy)]
struct ToggleGroupCtx {
    value: Signal<String>,
    on_change: Callback<String>,
    variant: ToggleVariant,
    size: ToggleSize,
}

#[component]
pub fn ToggleGroup(
    #[prop(into)] value: Signal<String>,
    #[prop(into)] on_change: Callback<String>,
    #[prop(optional)] variant: ToggleVariant,
    #[prop(optional)] size: ToggleSize,
    #[prop(optional, into)] class: String,
    children: Children,
) -> impl IntoView {
    provide_context(ToggleGroupCtx {
        value,
        on_change,
        variant,
        size,
    });
    // shadcn gates `shadow-xs` on `data-[variant=outline]`; the variant is known
    // statically here, so resolve it in Rust instead of carrying a data attribute.
    let shadow_class = if variant == ToggleVariant::Outline {
        "shadow-xs"
    } else {
        ""
    };
    view! {
        <div
            role="group"
            class=format!(
                "group/toggle-group flex w-fit items-center rounded-md {shadow_class} {class}",
            )
        >
            {children()}
        </div>
    }
}

#[component]
pub fn ToggleGroupItem(
    #[prop(into)] value: String,
    #[prop(optional, into)] disabled: MaybeProp<bool>,
    #[prop(optional, into)] class: String,
    children: Children,
) -> impl IntoView {
    let ctx = expect_context::<ToggleGroupCtx>();
    let variant_class = ctx.variant.classes();
    let size_class = ctx.size.classes();
    // shadcn gates these border rules on `data-[variant=outline]` (spacing=0); the
    // variant is known statically, so resolve it in Rust. Non-outline has no border,
    // making them no-ops, but we keep parity explicit.
    let border_class = if ctx.variant == ToggleVariant::Outline {
        "border-l-0 first:border-l"
    } else {
        ""
    };
    let selected = {
        let v = value.clone();
        Memo::new(move |_| ctx.value.get() == v)
    };
    let is_disabled = move || disabled.get().unwrap_or(false);
    let click_value = value;
    view! {
        <button
            type="button"
            aria-pressed=move || selected.get().to_string()
            data-state=move || if selected.get() { "on" } else { "off" }
            disabled=is_disabled
            on:click=move |_| {
                if !is_disabled() {
                    ctx.on_change.run(click_value.clone());
                }
            }
            class=move || {
                let on_class = if selected.get() { "bg-accent text-accent-foreground" } else { "" };
                format!(
                    "{TOGGLE_BASE} {variant_class} {size_class} w-auto min-w-0 shrink-0 px-3 focus:z-10 focus-visible:z-10 rounded-none shadow-none first:rounded-l-md last:rounded-r-md {border_class} {on_class} {class}",
                )
            }
        >
            {children()}
        </button>
    }
}
