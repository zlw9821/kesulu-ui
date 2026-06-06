use leptos::prelude::*;

#[component]
pub fn Empty(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    view! {
        <div class=format!(
            "flex min-w-0 flex-1 flex-col items-center justify-center gap-6 rounded-lg border border-dashed border-border p-6 text-center text-balance md:p-12 {}",
            class,
        )>{children()}</div>
    }
}

#[component]
pub fn EmptyHeader(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    view! {
        <div class=format!(
            "flex max-w-sm flex-col items-center gap-2 text-center {}",
            class,
        )>{children()}</div>
    }
}

#[derive(Default, Clone, Copy, PartialEq)]
pub enum EmptyMediaVariant {
    #[default]
    Default,
    Icon,
}

impl EmptyMediaVariant {
    fn classes(self) -> &'static str {
        match self {
            EmptyMediaVariant::Default => "bg-transparent",
            EmptyMediaVariant::Icon => {
                "flex size-10 shrink-0 items-center justify-center rounded-lg bg-muted text-foreground [&_svg:not([class*='size-'])]:size-6"
            }
        }
    }
}

#[component]
pub fn EmptyMedia(
    #[prop(optional)] variant: EmptyMediaVariant,
    #[prop(optional, into)] class: String,
    children: Children,
) -> impl IntoView {
    view! {
        <div class=format!(
            "mb-2 flex shrink-0 items-center justify-center [&_svg]:pointer-events-none [&_svg]:shrink-0 {} {}",
            variant.classes(),
            class,
        )>{children()}</div>
    }
}

#[component]
pub fn EmptyTitle(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    view! { <div class=format!("text-lg font-medium tracking-tight {}", class)>{children()}</div> }
}

#[component]
pub fn EmptyDescription(
    #[prop(optional, into)] class: String,
    children: Children,
) -> impl IntoView {
    view! {
        <div class=format!(
            "text-sm/relaxed text-muted-foreground [&>a]:underline [&>a]:underline-offset-4 [&>a:hover]:text-primary {}",
            class,
        )>{children()}</div>
    }
}

#[component]
pub fn EmptyContent(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    view! {
        <div class=format!(
            "flex w-full max-w-sm min-w-0 flex-col items-center gap-4 text-sm text-balance {}",
            class,
        )>{children()}</div>
    }
}
