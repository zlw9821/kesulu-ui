use leptos::prelude::*;

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
    /// Position, sizing, border and enter-animation classes for the content
    /// panel. Exit animations are dropped (see crate CONVENTIONS — `<Show>`
    /// unmounts immediately).
    fn classes(self) -> &'static str {
        match self {
            SheetSide::Right => {
                "inset-y-0 right-0 h-full w-3/4 border-l border-border \
                 animate-in slide-in-from-right duration-500 sm:max-w-sm"
            }
            SheetSide::Left => {
                "inset-y-0 left-0 h-full w-3/4 border-r border-border \
                 animate-in slide-in-from-left duration-500 sm:max-w-sm"
            }
            SheetSide::Top => {
                "inset-x-0 top-0 h-auto border-b border-border \
                 animate-in slide-in-from-top duration-500"
            }
            SheetSide::Bottom => {
                "inset-x-0 bottom-0 h-auto border-t border-border \
                 animate-in slide-in-from-bottom duration-500"
            }
        }
    }
}

#[component]
pub fn Sheet(#[prop(into)] open: RwSignal<bool>, children: ChildrenFn) -> impl IntoView {
    view! {
        <Show when=move || open.get()>
            // Overlay
            <div
                class="fixed inset-0 z-50 bg-black/50 animate-in fade-in-0"
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
                    "fixed z-50 flex flex-col gap-4 bg-background shadow-lg transition ease-in-out {} {}",
                    side.classes(),
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
