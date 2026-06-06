use leptos::children::ChildrenFn;
use leptos::prelude::*;

use super::presence::use_presence;

/// Trigger size — shadcn's `data-[size=…]` heights.
#[derive(Default, Clone, Copy, PartialEq)]
pub enum SelectSize {
    Sm,
    #[default]
    Default,
}

impl SelectSize {
    fn classes(self) -> &'static str {
        match self {
            SelectSize::Sm => "h-8",
            SelectSize::Default => "h-9",
        }
    }
}

/// Shared select state: the current value, the change emitter, and the open
/// popover flag. Consumed by every descendant via context (the project's
/// Radix-context analogue — see CONVENTIONS rule 5).
#[derive(Clone, Copy)]
struct SelectCtx {
    value: Signal<String>,
    on_change: Callback<String>,
    open: RwSignal<bool>,
}

#[component]
pub fn Select(
    #[prop(into)] value: Signal<String>,
    #[prop(into)] on_change: Callback<String>,
    #[prop(optional, into)] class: String,
    children: ChildrenFn,
) -> impl IntoView {
    let open = RwSignal::new(false);
    provide_context(SelectCtx {
        value,
        on_change,
        open,
    });
    view! { <div class=format!("relative inline-block {}", class)>{children()}</div> }
}

#[component]
pub fn SelectTrigger(
    #[prop(optional)] size: SelectSize,
    #[prop(optional, into)] disabled: MaybeProp<bool>,
    #[prop(optional, into)] class: String,
    children: Children,
) -> impl IntoView {
    let ctx = expect_context::<SelectCtx>();
    let is_disabled = move || disabled.get().unwrap_or(false);
    view! {
        <button
            type="button"
            disabled=is_disabled
            aria-expanded=move || ctx.open.get().to_string()
            on:click=move |_| {
                if !is_disabled() {
                    ctx.open.update(|v| *v = !*v);
                }
            }
            class=format!(
                "flex w-fit items-center justify-between gap-2 rounded-md border border-input bg-transparent px-3 py-2 text-sm whitespace-nowrap shadow-xs transition-[color,box-shadow] outline-none focus-visible:border-ring focus-visible:ring-[3px] focus-visible:ring-ring/50 disabled:cursor-not-allowed disabled:opacity-50 [&_svg]:pointer-events-none [&_svg]:shrink-0 [&_svg:not([class*='size-'])]:size-4 [&_svg:not([class*='text-'])]:text-muted-foreground {} {}",
                size.classes(),
                class,
            )
        >
            {children()}
            <svg
                class="size-4 opacity-50"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                stroke-width="2"
                stroke-linecap="round"
                stroke-linejoin="round"
            >
                <polyline points="6 9 12 15 18 9" />
            </svg>
        </button>
    }
}

/// Shows the current selected value, or `placeholder` (muted) when empty.
#[component]
pub fn SelectValue(#[prop(optional, into)] placeholder: String) -> impl IntoView {
    let ctx = expect_context::<SelectCtx>();
    let placeholder = StoredValue::new(placeholder);
    view! {
        <span class=move || {
            if ctx.value.get().is_empty() {
                "line-clamp-1 flex items-center gap-2 text-muted-foreground"
            } else {
                "line-clamp-1 flex items-center gap-2"
            }
        }>
            {move || {
                let v = ctx.value.get();
                if v.is_empty() { placeholder.get_value() } else { v }
            }}
        </span>
    }
}

#[component]
pub fn SelectContent(#[prop(optional, into)] class: String, children: ChildrenFn) -> impl IntoView {
    let ctx = expect_context::<SelectCtx>();
    let p = use_presence(ctx.open.into(), 150);
    let class = StoredValue::new(class);
    view! {
        <Show when=move || p.mounted.get()>
            // Click-outside-to-close backdrop (see dropdown.rs).
            <div class="fixed inset-0 z-40" on:click=move |_| ctx.open.set(false)></div>
            <div class=move || {
                format!(
                    "absolute left-0 top-full z-50 mt-1 max-h-96 min-w-[8rem] overflow-x-hidden overflow-y-auto rounded-md border border-border bg-popover text-popover-foreground shadow-md duration-150 {} {}",
                    if p.open.get() {
                        "animate-in fade-in-0 zoom-in-95 slide-in-from-top-2"
                    } else {
                        "animate-out fade-out-0 zoom-out-95 slide-out-to-top-2"
                    },
                    class.get_value(),
                )
            }>
                <div class="p-1">{children()}</div>
            </div>
        </Show>
    }
}

#[component]
pub fn SelectLabel(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    view! {
        <div class=format!("px-2 py-1.5 text-xs text-muted-foreground {}", class)>{children()}</div>
    }
}

#[component]
pub fn SelectGroup(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    view! {
        <div role="group" class=class>
            {children()}
        </div>
    }
}

#[component]
pub fn SelectItem(
    #[prop(into)] value: String,
    #[prop(optional, into)] disabled: MaybeProp<bool>,
    #[prop(optional, into)] class: String,
    children: Children,
) -> impl IntoView {
    let ctx = expect_context::<SelectCtx>();
    let selected = {
        let v = value.clone();
        Memo::new(move |_| ctx.value.get() == v)
    };
    let is_disabled = move || disabled.get().unwrap_or(false);
    let click_value = value;
    view! {
        <div
            role="option"
            aria-selected=move || selected.get().to_string()
            on:click=move |_| {
                if !is_disabled() {
                    ctx.on_change.run(click_value.clone());
                    ctx.open.set(false);
                }
            }
            class=move || {
                format!(
                    "relative flex w-full cursor-default items-center gap-2 rounded-sm py-1.5 pr-8 pl-2 text-sm outline-hidden select-none hover:bg-accent hover:text-accent-foreground [&_svg]:pointer-events-none [&_svg]:shrink-0 [&_svg:not([class*='size-'])]:size-4 [&_svg:not([class*='text-'])]:text-muted-foreground {} {}",
                    if is_disabled() { "pointer-events-none opacity-50" } else { "" },
                    class,
                )
            }
        >
            <span class="absolute right-2 flex size-3.5 items-center justify-center">
                <Show when=move || selected.get()>
                    <svg
                        class="size-4"
                        viewBox="0 0 24 24"
                        fill="none"
                        stroke="currentColor"
                        stroke-width="2"
                        stroke-linecap="round"
                        stroke-linejoin="round"
                    >
                        <polyline points="20 6 9 17 4 12" />
                    </svg>
                </Show>
            </span>
            {children()}
        </div>
    }
}

#[component]
pub fn SelectSeparator(#[prop(optional, into)] class: String) -> impl IntoView {
    view! { <div class=format!("pointer-events-none -mx-1 my-1 h-px bg-border {}", class)></div> }
}
