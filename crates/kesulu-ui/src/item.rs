use leptos::prelude::*;

/// Visual variant for [`Item`] — shadcn's `data-[variant=*]`.
#[derive(Default, Clone, Copy, PartialEq)]
pub enum ItemVariant {
    #[default]
    Default,
    Outline,
    Muted,
}

impl ItemVariant {
    fn classes(self) -> &'static str {
        match self {
            ItemVariant::Default => "bg-transparent",
            ItemVariant::Outline => "border-border",
            ItemVariant::Muted => "bg-muted/50",
        }
    }
}

/// Size for [`Item`] — shadcn's `data-[size=*]`.
#[derive(Default, Clone, Copy, PartialEq)]
pub enum ItemSize {
    #[default]
    Default,
    Sm,
}

impl ItemSize {
    fn classes(self) -> &'static str {
        match self {
            ItemSize::Default => "gap-4 p-4",
            ItemSize::Sm => "gap-2.5 px-4 py-3",
        }
    }
}

/// Visual variant for [`ItemMedia`] — shadcn's `data-[variant=*]`.
#[derive(Default, Clone, Copy, PartialEq)]
pub enum ItemMediaVariant {
    #[default]
    Default,
    Icon,
    Image,
}

impl ItemMediaVariant {
    fn classes(self) -> &'static str {
        match self {
            ItemMediaVariant::Default => "bg-transparent",
            ItemMediaVariant::Icon => {
                "size-8 rounded-sm border border-border bg-muted [&_svg:not([class*='size-'])]:size-4"
            }
            ItemMediaVariant::Image => {
                "size-10 overflow-hidden rounded-sm [&_img]:size-full [&_img]:object-cover"
            }
        }
    }
}

#[component]
pub fn ItemGroup(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    view! {
        <div role="list" class=format!("group/item-group flex flex-col {}", class)>
            {children()}
        </div>
    }
}

#[component]
pub fn ItemSeparator(#[prop(optional, into)] class: String) -> impl IntoView {
    view! {
        <div
            role="separator"
            class=format!("shrink-0 bg-border h-[1px] w-full my-0 {}", class)
        ></div>
    }
}

#[component]
pub fn Item(
    #[prop(optional)] variant: ItemVariant,
    #[prop(optional)] size: ItemSize,
    #[prop(optional, into)] class: String,
    children: Children,
) -> impl IntoView {
    let base = "group/item flex flex-wrap items-center rounded-md border border-transparent text-sm transition-colors duration-100 outline-none focus-visible:border-ring focus-visible:ring-[3px] focus-visible:ring-ring/50 [a]:transition-colors [a]:hover:bg-accent/50";
    view! {
        <div class=format!(
            "{} {} {} {}",
            base,
            variant.classes(),
            size.classes(),
            class,
        )>{children()}</div>
    }
}

#[component]
pub fn ItemMedia(
    #[prop(optional)] variant: ItemMediaVariant,
    #[prop(optional, into)] class: String,
    children: Children,
) -> impl IntoView {
    let base = "flex shrink-0 items-center justify-center gap-2 [&_svg]:pointer-events-none";
    view! { <div class=format!("{} {} {}", base, variant.classes(), class)>{children()}</div> }
}

#[component]
pub fn ItemContent(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    view! { <div class=format!("flex flex-1 flex-col gap-1 {}", class)>{children()}</div> }
}

#[component]
pub fn ItemTitle(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    view! {
        <div class=format!(
            "flex w-fit items-center gap-2 text-sm leading-snug font-medium {}",
            class,
        )>{children()}</div>
    }
}

#[component]
pub fn ItemDescription(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    view! {
        <p class=format!(
            "line-clamp-2 text-sm leading-normal font-normal text-balance text-muted-foreground [&>a]:underline [&>a]:underline-offset-4 [&>a:hover]:text-primary {}",
            class,
        )>{children()}</p>
    }
}

#[component]
pub fn ItemActions(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    view! { <div class=format!("flex items-center gap-2 {}", class)>{children()}</div> }
}

#[component]
pub fn ItemHeader(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    view! {
        <div class=format!(
            "flex basis-full items-center justify-between gap-2 {}",
            class,
        )>{children()}</div>
    }
}

#[component]
pub fn ItemFooter(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    view! {
        <div class=format!(
            "flex basis-full items-center justify-between gap-2 {}",
            class,
        )>{children()}</div>
    }
}
