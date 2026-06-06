use leptos::prelude::*;

/// Open/closed state shared by `CollapsibleTrigger` / `CollapsibleContent`
/// via context (Radix `Collapsible.Root` analogue — see CONVENTIONS rule 5).
#[derive(Clone, Copy)]
struct CollapsibleCtx {
    open: RwSignal<bool>,
}

#[component]
pub fn Collapsible(
    #[prop(into)] open: RwSignal<bool>,
    #[prop(optional, into)] class: String,
    children: Children,
) -> impl IntoView {
    provide_context(CollapsibleCtx { open });
    view! { <div class=class>{children()}</div> }
}

#[component]
pub fn CollapsibleTrigger(
    #[prop(optional, into)] class: String,
    children: Children,
) -> impl IntoView {
    let ctx = expect_context::<CollapsibleCtx>();
    view! {
        <button
            type="button"
            aria-expanded=move || ctx.open.get().to_string()
            on:click=move |_| ctx.open.update(|o| *o = !*o)
            class=class
        >
            {children()}
        </button>
    }
}

#[component]
pub fn CollapsibleContent(
    #[prop(optional, into)] class: String,
    children: ChildrenFn,
) -> impl IntoView {
    let ctx = expect_context::<CollapsibleCtx>();
    view! {
        <Show when=move || ctx.open.get()>
            <div class=format!(
                "overflow-hidden animate-in fade-in-0 slide-in-from-top-1 {}",
                class,
            )>{children()}</div>
        </Show>
    }
}
