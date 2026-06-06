use leptos::prelude::*;

/// Single-open accordion state, shared with every item via context.
#[derive(Clone, Copy)]
struct AccordionCtx {
    open: RwSignal<Option<String>>,
}

/// Per-item value, provided by `AccordionItem` to its `AccordionTrigger` /
/// `AccordionContent` children (nested context — see CONVENTIONS rule 5).
#[derive(Clone)]
struct AccordionItemCtx {
    value: String,
}

#[component]
pub fn Accordion(
    #[prop(optional, into)] default_value: String,
    #[prop(optional, into)] class: String,
    children: Children,
) -> impl IntoView {
    let initial = if default_value.is_empty() {
        None
    } else {
        Some(default_value)
    };
    provide_context(AccordionCtx {
        open: RwSignal::new(initial),
    });
    view! {
        <div class=format!(
            "divide-y divide-border rounded-md border border-border {}",
            class,
        )>{children()}</div>
    }
}

#[component]
pub fn AccordionItem(
    #[prop(into)] value: String,
    #[prop(optional, into)] class: String,
    children: Children,
) -> impl IntoView {
    provide_context(AccordionItemCtx { value });
    view! { <div class=class>{children()}</div> }
}

#[component]
pub fn AccordionTrigger(
    #[prop(optional, into)] class: String,
    children: Children,
) -> impl IntoView {
    let acc = expect_context::<AccordionCtx>();
    let value = expect_context::<AccordionItemCtx>().value;
    let is_open = {
        let v = value.clone();
        Memo::new(move |_| acc.open.get().as_deref() == Some(v.as_str()))
    };
    let toggle_value = value;
    view! {
        <button
            type="button"
            aria-expanded=move || is_open.get().to_string()
            on:click=move |_| {
                let v = toggle_value.clone();
                acc.open
                    .update(|cur| {
                        if cur.as_deref() == Some(v.as_str()) {
                            *cur = None;
                        } else {
                            *cur = Some(v);
                        }
                    });
            }
            class=format!(
                "flex w-full items-center justify-between py-4 px-4 text-sm font-medium text-left transition-all hover:underline {}",
                class,
            )
        >
            {children()}
            <svg
                class=move || {
                    format!(
                        "h-4 w-4 shrink-0 text-muted-foreground transition-transform duration-200 {}",
                        if is_open.get() { "rotate-180" } else { "" },
                    )
                }
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

#[component]
pub fn AccordionContent(
    #[prop(optional, into)] class: String,
    children: ChildrenFn,
) -> impl IntoView {
    let acc = expect_context::<AccordionCtx>();
    let value = expect_context::<AccordionItemCtx>().value;
    let is_open = Memo::new(move |_| acc.open.get().as_deref() == Some(value.as_str()));
    view! {
        <Show when=move || is_open.get()>
            <div class=format!(
                "px-4 pb-4 text-sm text-muted-foreground {}",
                class,
            )>{children()}</div>
        </Show>
    }
}
