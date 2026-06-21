use leptos::prelude::*;

#[component]
pub fn Kbd(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    view! {
        <kbd class=format!(
            "pointer-events-none inline-flex h-5 w-fit min-w-5 items-center justify-center gap-1 rounded-sm bg-muted px-1 font-sans text-xs font-medium text-muted-foreground select-none [&_svg:not([class*='size-'])]:size-3 {}",
            class,
        )>{children()}</kbd>
    }
}

#[component]
pub fn KbdGroup(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    view! { <kbd class=format!("inline-flex items-center gap-1 {}", class)>{children()}</kbd> }
}
