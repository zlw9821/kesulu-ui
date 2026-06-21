use std::sync::Arc;

use leptos::prelude::*;

use super::button::{Button, ButtonVariant};
use super::presence::use_presence;

#[derive(Clone, Copy)]
pub struct DialogState(pub RwSignal<bool>);

impl Default for DialogState {
    fn default() -> Self {
        Self(RwSignal::new(false))
    }
}

impl DialogState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn open(&self) {
        self.0.set(true);
    }

    pub fn close(&self) {
        self.0.set(false);
    }

    pub fn is_open(&self) -> bool {
        self.0.get()
    }
}

#[component]
pub fn Dialog(#[prop(into)] open: RwSignal<bool>, children: ChildrenFn) -> impl IntoView {
    let p = use_presence(open.into(), 200);
    view! {
        <Show when=move || p.mounted.get()>
            <div class="fixed inset-0 z-50">
                // Overlay
                <div
                    class=move || {
                        format!(
                            "fixed inset-0 bg-black/80 duration-200 anim-fill-forwards {}",
                            if p.open.get() {
                                "animate-in fade-in-0"
                            } else {
                                "animate-out fade-out-0"
                            },
                        )
                    }
                    on:click=move |_| open.set(false)
                ></div>
                // Content container
                <div class="fixed inset-0 flex items-center justify-center p-4">
                    <div
                        class=move || {
                            format!(
                                "relative z-50 w-full max-w-lg rounded-lg border border-border bg-card p-6 shadow-lg duration-200 anim-fill-forwards {}",
                                if p.open.get() {
                                    "animate-in fade-in-0 zoom-in-95"
                                } else {
                                    "animate-out fade-out-0 zoom-out-95"
                                },
                            )
                        }
                        on:click=|ev| ev.stop_propagation()
                    >
                        {children()}
                    </div>
                </div>
            </div>
        </Show>
    }
}

#[component]
pub fn DialogHeader(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    view! {
        <div class=format!(
            "flex flex-col space-y-1.5 text-center sm:text-left mb-4 {}",
            class,
        )>{children()}</div>
    }
}

#[component]
pub fn DialogTitle(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    view! {
        <h2 class=format!(
            "text-lg font-semibold leading-none tracking-tight text-foreground {}",
            class,
        )>{children()}</h2>
    }
}

#[component]
pub fn DialogDescription(
    #[prop(optional, into)] class: String,
    children: Children,
) -> impl IntoView {
    view! { <p class=format!("text-sm text-muted-foreground {}", class)>{children()}</p> }
}

#[component]
pub fn DialogFooter(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    view! {
        <div class=format!(
            "flex flex-col-reverse sm:flex-row sm:justify-end gap-2 mt-6 {}",
            class,
        )>{children()}</div>
    }
}

#[component]
pub fn AlertDialog(
    #[prop(into)] open: RwSignal<bool>,
    #[prop(into)] title: String,
    #[prop(into)] description: String,
    #[prop(optional, into)] confirm_label: String,
    #[prop(optional)] variant: ButtonVariant,
    on_confirm: Arc<dyn Fn() + Send + Sync>,
) -> impl IntoView {
    let confirm_text = StoredValue::new(if confirm_label.is_empty() {
        "Continue".to_string()
    } else {
        confirm_label
    });
    let title = StoredValue::new(title);
    let description = StoredValue::new(description);
    let p = use_presence(open.into(), 200);
    view! {
        <Show when=move || p.mounted.get()>
            <div class="fixed inset-0 z-50">
                <div class=move || {
                    format!(
                        "fixed inset-0 bg-black/80 duration-200 anim-fill-forwards {}",
                        if p.open.get() { "animate-in fade-in-0" } else { "animate-out fade-out-0" },
                    )
                }></div>
                <div class="fixed inset-0 flex items-center justify-center p-4">
                    <div class=move || {
                        format!(
                            "relative z-50 w-full max-w-md rounded-lg border border-border bg-card p-6 shadow-lg duration-200 anim-fill-forwards {}",
                            if p.open.get() {
                                "animate-in fade-in-0 zoom-in-95"
                            } else {
                                "animate-out fade-out-0 zoom-out-95"
                            },
                        )
                    }>
                        <div class="flex flex-col space-y-2 text-center sm:text-left">
                            <h2 class="text-lg font-semibold text-foreground">
                                {title.get_value()}
                            </h2>
                            <p class="text-sm text-muted-foreground">{description.get_value()}</p>
                        </div>
                        <div class="flex flex-col-reverse sm:flex-row sm:justify-end gap-2 mt-6">
                            <Button
                                variant=ButtonVariant::Outline
                                on:click=move |_| open.set(false)
                            >
                                "Cancel"
                            </Button>
                            <Button
                                variant=variant
                                on:click={
                                    let on_confirm = on_confirm.clone();
                                    move |_| {
                                        on_confirm();
                                        open.set(false);
                                    }
                                }
                            >
                                {confirm_text.get_value()}
                            </Button>
                        </div>
                    </div>
                </div>
            </div>
        </Show>
    }
}
