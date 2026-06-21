use leptos::prelude::*;

/// Avatar size (the shadcn `data-size` analogue → `&'static str` classes).
#[derive(Default, Clone, Copy, PartialEq)]
pub enum AvatarSize {
    Sm,
    #[default]
    Default,
    Lg,
}

impl AvatarSize {
    /// Root `size-*` class.
    fn root(self) -> &'static str {
        match self {
            AvatarSize::Sm => "size-6",
            AvatarSize::Default => "size-8",
            AvatarSize::Lg => "size-10",
        }
    }

    /// Fallback text size (shadcn shrinks to `text-xs` at the `sm` size).
    fn fallback_text(self) -> &'static str {
        match self {
            AvatarSize::Sm => "text-xs",
            _ => "text-sm",
        }
    }

    /// Badge dot size + inner-svg size (shadcn `group-data-[size=*]` rules).
    fn badge(self) -> &'static str {
        match self {
            AvatarSize::Sm => "size-2 [&>svg]:hidden",
            AvatarSize::Default => "size-2.5 [&>svg]:size-2",
            AvatarSize::Lg => "size-3 [&>svg]:size-2",
        }
    }
}

/// Provided by `Avatar` so `AvatarFallback` / `AvatarBadge` can react to the
/// size without cross-component `group-data-*` selectors.
#[derive(Clone, Copy)]
struct AvatarCtx {
    size: AvatarSize,
}

#[component]
pub fn Avatar(
    #[prop(optional)] size: AvatarSize,
    #[prop(optional, into)] class: String,
    children: Children,
) -> impl IntoView {
    provide_context(AvatarCtx { size });
    let size_class = size.root();
    view! {
        <span class=format!(
            "group/avatar relative flex shrink-0 overflow-hidden rounded-full select-none {size_class} {class}",
        )>{children()}</span>
    }
}

#[component]
pub fn AvatarImage(
    #[prop(into)] src: String,
    #[prop(optional, into)] alt: String,
    #[prop(optional, into)] class: String,
) -> impl IntoView {
    view! { <img src=src alt=alt class=format!("aspect-square size-full {class}") /> }
}

#[component]
pub fn AvatarFallback(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    let text_class = use_context::<AvatarCtx>()
        .map(|ctx| ctx.size.fallback_text())
        .unwrap_or("text-sm");
    view! {
        <span class=format!(
            "flex size-full items-center justify-center rounded-full bg-muted text-muted-foreground {text_class} {class}",
        )>{children()}</span>
    }
}

#[component]
pub fn AvatarBadge(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    let badge_class = use_context::<AvatarCtx>()
        .map(|ctx| ctx.size.badge())
        .unwrap_or("size-2.5 [&>svg]:size-2");
    view! {
        <span class=format!(
            "absolute right-0 bottom-0 z-10 inline-flex items-center justify-center rounded-full bg-primary text-primary-foreground ring-2 ring-background select-none {badge_class} {class}",
        )>{children()}</span>
    }
}

#[component]
pub fn AvatarGroup(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    view! {
        <div class=format!(
            "group/avatar-group flex -space-x-2 [&>span]:ring-2 [&>span]:ring-background {class}",
        )>{children()}</div>
    }
}

#[component]
pub fn AvatarGroupCount(
    #[prop(optional)] size: AvatarSize,
    #[prop(optional, into)] class: String,
    children: Children,
) -> impl IntoView {
    let size_class = size.root();
    view! {
        <div class=format!(
            "relative flex shrink-0 items-center justify-center rounded-full bg-muted text-sm text-muted-foreground ring-2 ring-background {size_class} {class}",
        )>{children()}</div>
    }
}
