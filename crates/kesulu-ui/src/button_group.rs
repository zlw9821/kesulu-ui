use leptos::prelude::*;

/// Layout axis for a [`ButtonGroup`] (shadcn `data-[orientation=*]`).
#[derive(Default, Clone, Copy, PartialEq)]
pub enum ButtonGroupOrientation {
    #[default]
    Horizontal,
    Vertical,
}

impl ButtonGroupOrientation {
    fn classes(self) -> &'static str {
        match self {
            // Collapse shared borders & rounding between adjacent children.
            ButtonGroupOrientation::Horizontal => {
                "[&>*:not(:first-child)]:rounded-l-none [&>*:not(:first-child)]:border-l-0 [&>*:not(:last-child)]:rounded-r-none"
            }
            ButtonGroupOrientation::Vertical => {
                "flex-col [&>*:not(:first-child)]:rounded-t-none [&>*:not(:first-child)]:border-t-0 [&>*:not(:last-child)]:rounded-b-none"
            }
        }
    }
}

#[component]
pub fn ButtonGroup(
    #[prop(optional)] orientation: ButtonGroupOrientation,
    #[prop(optional, into)] class: String,
    children: Children,
) -> impl IntoView {
    let base = "flex w-fit items-stretch [&>*]:focus-visible:relative [&>*]:focus-visible:z-10 [&>input]:flex-1";
    view! {
        <div role="group" class=format!("{} {} {}", base, orientation.classes(), class)>
            {children()}
        </div>
    }
}

#[component]
pub fn ButtonGroupText(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    view! {
        <div class=format!(
            "flex items-center gap-2 rounded-md border border-input bg-muted px-4 text-sm font-medium shadow-xs [&_svg]:pointer-events-none [&_svg:not([class*='size-'])]:size-4 {}",
            class,
        )>{children()}</div>
    }
}

/// Axis of a [`ButtonGroupSeparator`] (shadcn `data-[orientation=*]`).
#[derive(Default, Clone, Copy, PartialEq)]
pub enum ButtonGroupSeparatorOrientation {
    #[default]
    Vertical,
    Horizontal,
}

impl ButtonGroupSeparatorOrientation {
    fn classes(self) -> &'static str {
        match self {
            ButtonGroupSeparatorOrientation::Vertical => "h-auto w-px",
            ButtonGroupSeparatorOrientation::Horizontal => "h-px w-full",
        }
    }
}

#[component]
pub fn ButtonGroupSeparator(
    #[prop(optional)] orientation: ButtonGroupSeparatorOrientation,
    #[prop(optional, into)] class: String,
) -> impl IntoView {
    view! {
        <div
            role="separator"
            class=format!(
                "relative shrink-0 self-stretch bg-input {} {}",
                orientation.classes(),
                class,
            )
        ></div>
    }
}
