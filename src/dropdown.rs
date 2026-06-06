use leptos::prelude::*;

use super::presence::use_presence;

#[component]
pub fn DropdownMenu(
    #[prop(into)] trigger: AnyView,
    #[prop(optional, into)] class: String,
    children: ChildrenFn,
) -> impl IntoView {
    let (open, set_open) = signal(false);
    let p = use_presence(open.into(), 150);
    view! {
        <div class=format!("relative inline-block {}", class)>
            <div on:click=move |_| set_open.update(|v| *v = !*v)>{trigger}</div>
            <Show when=move || p.mounted.get()>
                <div class="fixed inset-0 z-40" on:click=move |_| set_open.set(false)></div>
                <div class=move || {
                    format!(
                        "absolute right-0 top-full z-50 mt-1 min-w-[8rem] overflow-hidden rounded-md border border-border bg-popover p-1 text-popover-foreground shadow-md duration-150 {}",
                        if p.open.get() {
                            "animate-in fade-in-0 zoom-in-95"
                        } else {
                            "animate-out fade-out-0 zoom-out-95"
                        },
                    )
                }>{children()}</div>
            </Show>
        </div>
    }
}

#[component]
pub fn DropdownMenuItem(
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] on_click: Option<Callback<()>>,
    children: Children,
) -> impl IntoView {
    view! {
        <button
            class=format!(
                "relative flex w-full cursor-pointer select-none items-center rounded-sm px-2 py-1.5 text-sm outline-none transition-colors hover:bg-secondary hover:text-foreground focus:bg-secondary {}",
                class,
            )
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
pub fn DropdownMenuSeparator() -> impl IntoView {
    view! { <div class="-mx-1 my-1 h-px bg-border"></div> }
}
