use leptos::prelude::*;

/// Layout orientation of an [`InputGroup`]. Horizontal is the default; switch to
/// [`InputGroupOrientation::Block`] when the group contains block-aligned addons
/// (the shadcn `has-[>[data-align=block-*]]` column layout) so the wrapper stacks
/// its rows vertically and grows to fit.
#[derive(Default, Clone, Copy, PartialEq)]
pub enum InputGroupOrientation {
    #[default]
    Horizontal,
    Block,
}

impl InputGroupOrientation {
    fn classes(self) -> &'static str {
        match self {
            // shadcn base: fixed control height, inline addons.
            InputGroupOrientation::Horizontal => "h-9 items-center",
            // shadcn has-[>[data-align=block-*]]: auto-height column stack.
            InputGroupOrientation::Block => "h-auto flex-col items-stretch",
        }
    }
}

/// Placement of an [`InputGroupAddon`] relative to the control — the Rust
/// analogue of shadcn's `data-align` (`inline-start` / `inline-end` /
/// `block-start` / `block-end`).
#[derive(Default, Clone, Copy, PartialEq)]
pub enum InputGroupAddonAlign {
    #[default]
    InlineStart,
    InlineEnd,
    BlockStart,
    BlockEnd,
}

impl InputGroupAddonAlign {
    fn classes(self) -> &'static str {
        match self {
            InputGroupAddonAlign::InlineStart => "order-first pl-3",
            InputGroupAddonAlign::InlineEnd => "order-last pr-3",
            InputGroupAddonAlign::BlockStart => "order-first w-full justify-start px-3 pt-3",
            InputGroupAddonAlign::BlockEnd => "order-last w-full justify-start px-3 pb-3",
        }
    }
}

/// Size of an [`InputGroupButton`] — shadcn's `xs` / `sm` / `icon-xs` / `icon-sm`.
#[derive(Default, Clone, Copy, PartialEq)]
pub enum InputGroupButtonSize {
    #[default]
    Xs,
    Sm,
    IconXs,
    IconSm,
}

impl InputGroupButtonSize {
    fn classes(self) -> &'static str {
        match self {
            InputGroupButtonSize::Xs => {
                "h-6 gap-1 rounded-sm px-2 [&>svg:not([class*='size-'])]:size-3.5"
            }
            InputGroupButtonSize::Sm => "h-8 gap-1.5 rounded-md px-2.5",
            InputGroupButtonSize::IconXs => "size-6 rounded-sm p-0",
            InputGroupButtonSize::IconSm => "size-8 p-0",
        }
    }
}

#[component]
pub fn InputGroup(
    #[prop(optional)] orientation: InputGroupOrientation,
    #[prop(optional, into)] class: String,
    children: Children,
) -> impl IntoView {
    view! {
        <div
            role="group"
            class=format!(
                "group/input-group relative flex w-full min-w-0 rounded-md border border-input shadow-xs transition-[color,box-shadow] outline-none has-[:focus-visible]:border-ring has-[:focus-visible]:ring-[3px] has-[:focus-visible]:ring-ring/50 has-[[aria-invalid=true]]:border-destructive has-[[aria-invalid=true]]:ring-destructive/20 {} {}",
                orientation.classes(),
                class,
            )
        >
            {children()}
        </div>
    }
}

#[component]
pub fn InputGroupAddon(
    #[prop(optional)] align: InputGroupAddonAlign,
    #[prop(optional, into)] class: String,
    children: Children,
) -> impl IntoView {
    view! {
        <div
            role="group"
            class=format!(
                "flex h-auto cursor-text items-center justify-center gap-2 py-1.5 text-sm font-medium text-muted-foreground select-none [&>kbd]:rounded-sm [&>svg:not([class*='size-'])]:size-4 {} {}",
                align.classes(),
                class,
            )
        >
            {children()}
        </div>
    }
}

#[component]
pub fn InputGroupButton(
    #[prop(optional)] size: InputGroupButtonSize,
    #[prop(optional, into)] disabled: MaybeProp<bool>,
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] on_click: Option<Callback<()>>,
    children: Children,
) -> impl IntoView {
    // shadcn defaults this button to the `ghost` variant; mirror those classes
    // inline so this stays a bare element (no cross-module Button dependency).
    let base = "inline-flex items-center justify-center gap-2 text-sm font-medium shadow-none transition-colors hover:bg-secondary hover:text-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring disabled:pointer-events-none disabled:opacity-50";
    view! {
        <button
            type="button"
            disabled=move || disabled.get().unwrap_or(false)
            class=format!("{} {} {}", base, size.classes(), class)
            on:click=move |_| {
                if let Some(cb) = &on_click {
                    cb.run(());
                }
            }
        >
            {children()}
        </button>
    }
}

#[component]
pub fn InputGroupText(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    view! {
        <span class=format!(
            "flex items-center gap-2 text-sm text-muted-foreground [&_svg]:pointer-events-none [&_svg:not([class*='size-'])]:size-4 {}",
            class,
        )>{children()}</span>
    }
}

#[component]
pub fn InputGroupInput(
    #[prop(into)] value: Signal<String>,
    #[prop(into)] on_input: Callback<String>,
    #[prop(optional, into)] placeholder: String,
    #[prop(optional, into)] input_type: String,
    #[prop(optional, into)] disabled: MaybeProp<bool>,
    #[prop(optional, into)] class: String,
) -> impl IntoView {
    let input_type = if input_type.is_empty() {
        "text".to_string()
    } else {
        input_type
    };
    view! {
        <input
            type=input_type
            prop:value=value
            placeholder=placeholder
            disabled=move || disabled.get().unwrap_or(false)
            class=format!(
                "flex-1 min-w-0 rounded-none border-0 bg-transparent px-2 py-1 text-sm text-foreground shadow-none outline-none placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-0 disabled:cursor-not-allowed disabled:opacity-50 {}",
                class,
            )
            on:input=move |ev| {
                on_input.run(event_target_value(&ev));
            }
        />
    }
}

#[component]
pub fn InputGroupTextarea(
    #[prop(into)] value: Signal<String>,
    #[prop(into)] on_input: Callback<String>,
    #[prop(optional, into)] placeholder: String,
    #[prop(optional, default = 3)] rows: u32,
    #[prop(optional, into)] disabled: MaybeProp<bool>,
    #[prop(optional, into)] class: String,
) -> impl IntoView {
    view! {
        <textarea
            rows=rows
            placeholder=placeholder
            disabled=move || disabled.get().unwrap_or(false)
            prop:value=value
            class=format!(
                "flex-1 min-w-0 resize-none rounded-none border-0 bg-transparent px-3 py-3 text-sm text-foreground shadow-none outline-none placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-0 disabled:cursor-not-allowed disabled:opacity-50 {}",
                class,
            )
            on:input=move |ev| {
                on_input.run(event_target_value(&ev));
            }
        />
    }
}
