use leptos::prelude::*;

use super::button::{Button, ButtonSize, ButtonVariant};
use super::presence::use_presence;

/// Sticky "unsaved changes" action bar. Slides up from the bottom edge when
/// `visible` turns true and slides back out when it clears (presence-animated).
/// The `Save` button shows a spinner while `saving`. Pair it with a success
/// toast at the call site for a complete save-confirmation loop.
#[component]
pub fn SaveBar(
    #[prop(into)] visible: Signal<bool>,
    #[prop(optional, into)] saving: MaybeProp<bool>,
    #[prop(into)] on_discard: Callback<()>,
    #[prop(into)] on_save: Callback<()>,
    /// Left-side status text (default: "Unsaved changes").
    #[prop(optional, into)]
    message: String,
    /// Save button label (default: "Save changes").
    #[prop(optional, into)]
    save_label: String,
) -> impl IntoView {
    let p = use_presence(visible, 200);
    let message = StoredValue::new(if message.is_empty() {
        "Unsaved changes".to_string()
    } else {
        message
    });
    let save_label = StoredValue::new(if save_label.is_empty() {
        "Save changes".to_string()
    } else {
        save_label
    });
    view! {
        <Show when=move || p.mounted.get()>
            <div class=move || {
                format!(
                    "fixed inset-x-0 bottom-0 z-50 border-t border-border bg-card/95 backdrop-blur-sm shadow-[0_-4px_16px_rgba(0,0,0,0.18)] duration-200 {}",
                    if p.open.get() {
                        "animate-in fade-in-0 slide-in-from-bottom"
                    } else {
                        "animate-out fade-out-0 slide-out-to-bottom"
                    },
                )
            }>
                <div class="mx-auto flex max-w-[1080px] items-center gap-4 px-7 py-2.5">
                    <span class="flex items-center gap-2 text-xs text-muted-foreground">
                        <span class="size-1.5 shrink-0 rounded-full bg-warning animate-pulse"></span>
                        {move || message.get_value()}
                    </span>
                    <div class="flex-1"></div>
                    <Button
                        variant=ButtonVariant::Ghost
                        size=ButtonSize::Sm
                        on:click=move |_| on_discard.run(())
                    >
                        "Discard"
                    </Button>
                    <Button size=ButtonSize::Sm loading=saving on:click=move |_| on_save.run(())>
                        {move || save_label.get_value()}
                    </Button>
                </div>
            </div>
        </Show>
    }
}
