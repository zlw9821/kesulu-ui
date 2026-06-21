use leptos::prelude::*;

use super::presence::use_presence;

/// Shared open state for a `Popover`, consumed by `PopoverTrigger` /
/// `PopoverContent` via context (the project's Radix-context analogue — see
/// CONVENTIONS rule 5).
#[derive(Clone, Copy)]
struct PopoverCtx {
    open: RwSignal<bool>,
}

#[component]
pub fn Popover(#[prop(into)] open: RwSignal<bool>, children: ChildrenFn) -> impl IntoView {
    provide_context(PopoverCtx { open });
    view! { <div class="relative inline-block">{children()}</div> }
}

#[component]
pub fn PopoverTrigger(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    let ctx = expect_context::<PopoverCtx>();
    view! {
        <div
            class=format!("inline-block {}", class)
            on:click=move |_| ctx.open.update(|v| *v = !*v)
        >
            {children()}
        </div>
    }
}

#[component]
pub fn PopoverContent(
    #[prop(optional, into)] class: String,
    children: ChildrenFn,
) -> impl IntoView {
    let ctx = expect_context::<PopoverCtx>();
    let p = use_presence(ctx.open.into(), 150);
    let class = StoredValue::new(class);
    view! {
        <Show when=move || p.mounted.get()>
            // Click-outside overlay (see dropdown.rs).
            <div class="fixed inset-0 z-40" on:click=move |_| ctx.open.set(false)></div>
            <div
                class=move || {
                    format!(
                        "absolute left-1/2 top-full z-50 mt-1 w-72 -translate-x-1/2 rounded-md border border-border bg-popover p-4 text-popover-foreground shadow-md outline-none duration-150 {} {}",
                        if p.open.get() {
                            "animate-in fade-in-0 zoom-in-95"
                        } else {
                            "animate-out fade-out-0 zoom-out-95"
                        },
                        class.get_value(),
                    )
                }
                on:click=|ev| ev.stop_propagation()
            >
                {children()}
            </div>
        </Show>
    }
}

#[component]
pub fn PopoverHeader(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    view! { <div class=format!("flex flex-col gap-1 text-sm {}", class)>{children()}</div> }
}

#[component]
pub fn PopoverTitle(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    view! { <div class=format!("font-medium {}", class)>{children()}</div> }
}

#[component]
pub fn PopoverDescription(
    #[prop(optional, into)] class: String,
    children: Children,
) -> impl IntoView {
    view! { <p class=format!("text-muted-foreground {}", class)>{children()}</p> }
}
