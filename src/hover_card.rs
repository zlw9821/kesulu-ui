use leptos::prelude::*;

/// Horizontal alignment of the hover-card content relative to the trigger.
/// Mirrors Radix's `align` prop (`start` / `center` / `end`).
#[derive(Default, Clone, Copy, PartialEq)]
pub enum HoverCardAlign {
    Start,
    #[default]
    Center,
    End,
}

impl HoverCardAlign {
    fn classes(self) -> &'static str {
        match self {
            HoverCardAlign::Start => "left-0",
            HoverCardAlign::Center => "left-1/2 -translate-x-1/2",
            HoverCardAlign::End => "right-0",
        }
    }
}

/// Root wrapper. Establishes the hover group + positioning context so the
/// content can anchor against the trigger. No Radix `Root` state is needed —
/// visibility is driven purely by CSS `group-hover` (matching the existing
/// `Tooltip` in `feedback.rs`).
#[component]
pub fn HoverCard(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    view! { <div class=format!("relative inline-block group {}", class)>{children()}</div> }
}

/// The element users hover to reveal the card.
#[component]
pub fn HoverCardTrigger(
    #[prop(optional, into)] class: String,
    children: Children,
) -> impl IntoView {
    view! { <span class=format!("inline-block {}", class)>{children()}</span> }
}

/// The floating card revealed on hover. Card-styled popover with shadcn's
/// enter animation; positioned below the trigger and aligned via `align`.
#[component]
pub fn HoverCardContent(
    #[prop(optional)] align: HoverCardAlign,
    #[prop(optional, into)] class: String,
    children: Children,
) -> impl IntoView {
    view! {
        <div class=format!(
            "invisible absolute top-full {} z-50 mt-1 w-64 rounded-md border border-border bg-popover p-4 text-popover-foreground shadow-md outline-none opacity-0 transition-opacity group-hover:visible group-hover:opacity-100 group-hover:animate-in group-hover:fade-in-0 group-hover:zoom-in-95 group-hover:slide-in-from-top-2 {}",
            align.classes(),
            class,
        )>{children()}</div>
    }
}
