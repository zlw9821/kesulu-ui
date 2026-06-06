use leptos::prelude::*;

use super::presence::use_presence;

/// Edge the sheet slides in from. Mirrors shadcn's `side` prop.
#[derive(Default, Clone, Copy, PartialEq)]
pub enum SheetSide {
    #[default]
    Right,
    Left,
    Top,
    Bottom,
}

impl SheetSide {
    /// Static position / sizing / border classes for the content panel.
    fn base(self) -> &'static str {
        match self {
            SheetSide::Right => "inset-y-0 right-0 h-full w-3/4 border-l border-border sm:max-w-sm",
            SheetSide::Left => "inset-y-0 left-0 h-full w-3/4 border-r border-border sm:max-w-sm",
            SheetSide::Top => "inset-x-0 top-0 h-auto border-b border-border",
            SheetSide::Bottom => "inset-x-0 bottom-0 h-auto border-t border-border",
        }
    }

    /// Direction-aware enter (`open`) / exit (`!open`) animation classes.
    fn anim(self, open: bool) -> &'static str {
        match (self, open) {
            (SheetSide::Right, true) => "animate-in slide-in-from-right",
            (SheetSide::Right, false) => "animate-out slide-out-to-right",
            (SheetSide::Left, true) => "animate-in slide-in-from-left",
            (SheetSide::Left, false) => "animate-out slide-out-to-left",
            (SheetSide::Top, true) => "animate-in slide-in-from-top",
            (SheetSide::Top, false) => "animate-out slide-out-to-top",
            (SheetSide::Bottom, true) => "animate-in slide-in-from-bottom",
            (SheetSide::Bottom, false) => "animate-out slide-out-to-bottom",
        }
    }
}

#[component]
pub fn Sheet(#[prop(into)] open: RwSignal<bool>, children: ChildrenFn) -> impl IntoView {
    let p = use_presence(open.into(), 500);
    view! {
        <Show when=move || p.mounted.get()>
            // Overlay
            <div
                class=move || {
                    format!(
                        "fixed inset-0 z-50 bg-black/50 {}",
                        if p.open.get() { "animate-in fade-in-0" } else { "animate-out fade-out-0" },
                    )
                }
                on:click=move |_| open.set(false)
            ></div>
            {children()}
        </Show>
    }
}

#[component]
pub fn SheetContent(
    #[prop(into)] open: RwSignal<bool>,
    #[prop(optional)] side: SheetSide,
    #[prop(optional, into)] show_close_button: MaybeProp<bool>,
    #[prop(optional, into)] class: String,
    children: ChildrenFn,
) -> impl IntoView {
    let class = StoredValue::new(class);
    let show_close = move || show_close_button.get().unwrap_or(true);
    view! {
        <div
            class=move || {
                format!(
                    "fixed z-50 flex flex-col gap-4 bg-background shadow-lg transition ease-in-out duration-500 {} {} {}",
                    side.base(),
                    side.anim(open.get()),
                    class.get_value(),
                )
            }
            on:click=|ev| ev.stop_propagation()
        >
            {children()}
            <Show when=show_close>
                <button
                    type="button"
                    on:click=move |_| open.set(false)
                    class="absolute top-4 right-4 rounded-xs opacity-70 ring-offset-background transition-opacity hover:opacity-100 focus:outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2 disabled:pointer-events-none"
                >
                    <svg
                        class="size-4"
                        viewBox="0 0 24 24"
                        fill="none"
                        stroke="currentColor"
                        stroke-width="2"
                        stroke-linecap="round"
                        stroke-linejoin="round"
                    >
                        <path d="M18 6 6 18" />
                        <path d="m6 6 12 12" />
                    </svg>
                    <span class="sr-only">"Close"</span>
                </button>
            </Show>
        </div>
    }
}

#[component]
pub fn SheetHeader(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    view! { <div class=format!("flex flex-col gap-1.5 p-4 {}", class)>{children()}</div> }
}

#[component]
pub fn SheetFooter(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    view! { <div class=format!("mt-auto flex flex-col gap-2 p-4 {}", class)>{children()}</div> }
}

#[component]
pub fn SheetTitle(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    view! { <h2 class=format!("font-semibold text-foreground {}", class)>{children()}</h2> }
}

#[component]
pub fn SheetDescription(
    #[prop(optional, into)] class: String,
    children: Children,
) -> impl IntoView {
    view! { <p class=format!("text-sm text-muted-foreground {}", class)>{children()}</p> }
}
