use leptos::prelude::*;

use super::icon::{Icon, IconName};

#[component]
pub fn Breadcrumb(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    view! {
        <nav aria-label="breadcrumb" class=class>
            {children()}
        </nav>
    }
}

#[component]
pub fn BreadcrumbList(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    view! {
        <ol class=format!(
            "flex flex-wrap items-center gap-1.5 text-sm break-words text-muted-foreground sm:gap-2.5 {}",
            class,
        )>{children()}</ol>
    }
}

#[component]
pub fn BreadcrumbItem(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    view! { <li class=format!("inline-flex items-center gap-1.5 {}", class)>{children()}</li> }
}

#[component]
pub fn BreadcrumbLink(
    #[prop(optional, into)] href: String,
    #[prop(optional, into)] class: String,
    children: Children,
) -> impl IntoView {
    view! {
        <a href=href class=format!("transition-colors hover:text-foreground {}", class)>
            {children()}
        </a>
    }
}

#[component]
pub fn BreadcrumbPage(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    view! {
        <span
            role="link"
            aria-disabled="true"
            aria-current="page"
            class=format!("font-normal text-foreground {}", class)
        >
            {children()}
        </span>
    }
}

#[component]
pub fn BreadcrumbSeparator(
    #[prop(optional)] children: Option<Children>,
    #[prop(optional, into)] class: String,
) -> impl IntoView {
    view! {
        <li role="presentation" aria-hidden="true" class=format!("[&>svg]:size-3.5 {}", class)>
            {match children {
                Some(children) => children().into_any(),
                None => view! { <Icon name=IconName::ChevronRight class="size-3.5" /> }.into_any(),
            }}
        </li>
    }
}

#[component]
pub fn BreadcrumbEllipsis(#[prop(optional, into)] class: String) -> impl IntoView {
    view! {
        <span
            role="presentation"
            aria-hidden="true"
            class=format!("flex size-9 items-center justify-center {}", class)
        >
            <Icon name=IconName::Ellipsis />
            <span class="sr-only">"More"</span>
        </span>
    }
}
