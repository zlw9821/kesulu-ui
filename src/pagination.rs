use leptos::prelude::*;

use super::icon::{Icon, IconName};

/// Root `<nav>` wrapper, centered and full-width.
#[component]
pub fn Pagination(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    view! {
        <nav
            role="navigation"
            aria-label="pagination"
            class=format!("mx-auto flex w-full justify-center {}", class)
        >
            {children()}
        </nav>
    }
}

/// `<ul>` row holding the pagination items.
#[component]
pub fn PaginationContent(
    #[prop(optional, into)] class: String,
    children: Children,
) -> impl IntoView {
    view! { <ul class=format!("flex flex-row items-center gap-1 {}", class)>{children()}</ul> }
}

/// `<li>` slot wrapping a single link / ellipsis.
#[component]
pub fn PaginationItem(children: Children) -> impl IntoView {
    view! { <li>{children()}</li> }
}

/// Button-ish link sizes, mirroring `Button`'s `icon` / `default` sizing.
#[derive(Default, Clone, Copy, PartialEq)]
pub enum PaginationLinkSize {
    #[default]
    Icon,
    Default,
}

impl PaginationLinkSize {
    fn classes(self) -> &'static str {
        match self {
            PaginationLinkSize::Icon => "h-9 w-9 rounded-md",
            PaginationLinkSize::Default => "h-9 px-4 py-2 text-sm rounded-md gap-2",
        }
    }
}

/// Anchor styled like a ghost (or outline when active) button.
#[component]
pub fn PaginationLink(
    #[prop(optional)] is_active: bool,
    #[prop(optional)] size: PaginationLinkSize,
    #[prop(optional, into)] href: String,
    #[prop(optional, into)] class: String,
    children: Children,
) -> impl IntoView {
    let base = "inline-flex items-center justify-center font-medium transition-colors duration-200 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 focus-visible:ring-offset-background";
    let variant_class = if is_active {
        "border border-input bg-background hover:bg-secondary hover:text-foreground shadow-sm"
    } else {
        "hover:bg-secondary hover:text-foreground"
    };
    let size_class = size.classes();
    let aria_current = if is_active { "page" } else { "" };
    view! {
        <a
            href=href
            aria-current=aria_current
            data-active=is_active.to_string()
            class=format!("{base} {variant_class} {size_class} {class}")
        >
            {children()}
        </a>
    }
}

/// "Previous" link with a leading chevron.
#[component]
pub fn PaginationPrevious(
    #[prop(optional, into)] href: String,
    #[prop(optional, into)] class: String,
) -> impl IntoView {
    view! {
        <PaginationLink
            href=href
            size=PaginationLinkSize::Default
            attr:aria-label="Go to previous page"
            class=format!("gap-1 px-2.5 sm:pl-2.5 {}", class)
        >
            <Icon name=IconName::ChevronLeft />
            <span class="hidden sm:block">Previous</span>
        </PaginationLink>
    }
}

/// "Next" link with a trailing chevron.
#[component]
pub fn PaginationNext(
    #[prop(optional, into)] href: String,
    #[prop(optional, into)] class: String,
) -> impl IntoView {
    view! {
        <PaginationLink
            href=href
            size=PaginationLinkSize::Default
            attr:aria-label="Go to next page"
            class=format!("gap-1 px-2.5 sm:pr-2.5 {}", class)
        >
            <span class="hidden sm:block">Next</span>
            <Icon name=IconName::ChevronRight />
        </PaginationLink>
    }
}

/// Non-interactive "more pages" ellipsis marker.
#[component]
pub fn PaginationEllipsis(#[prop(optional, into)] class: String) -> impl IntoView {
    view! {
        <span aria-hidden="true" class=format!("flex size-9 items-center justify-center {}", class)>
            <Icon name=IconName::Ellipsis />
            <span class="sr-only">More pages</span>
        </span>
    }
}
