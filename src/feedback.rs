use leptos::prelude::*;

#[component]
pub fn ErrorBanner(
    #[prop(into)] visible: Signal<bool>,
    #[prop(optional, into)] message: MaybeProp<String>,
    #[prop(optional, into)] on_retry: Option<Callback<()>>,
    #[prop(optional, into)] class: String,
) -> impl IntoView {
    view! {
        <Show when=move || visible.get()>
            <div class=format!(
                "flex items-center gap-3 px-4 py-3 rounded-lg border border-destructive/50 bg-destructive/10 text-destructive text-sm font-medium {}",
                class,
            )>
                <svg
                    class="w-4 h-4 shrink-0"
                    viewBox="0 0 24 24"
                    fill="none"
                    stroke="currentColor"
                    stroke-width="2"
                >
                    <circle cx="12" cy="12" r="10" />
                    <line x1="12" y1="8" x2="12" y2="12" />
                    <line x1="12" y1="16" x2="12.01" y2="16" />
                </svg>
                <span class="flex-1">{move || message.get().unwrap_or_default()}</span>
                {on_retry
                    .map(|cb| {
                        view! {
                            <button
                                type="button"
                                class="shrink-0 rounded-md border border-destructive/40 bg-destructive/10 px-2.5 py-1 text-[11px] font-bold uppercase tracking-wide hover:bg-destructive/20 transition-colors"
                                on:click=move |_| cb.run(())
                            >
                                "Retry"
                            </button>
                        }
                    })}
            </div>
        </Show>
    }
}

#[component]
pub fn SkeletonBlock(
    #[prop(optional, default = 3)] rows: usize,
    #[prop(optional, into)] class: String,
) -> impl IntoView {
    let outer_class = format!("space-y-3 animate-pulse {}", class);
    view! {
        <div class=outer_class>
            {(0..rows)
                .map(|i| {
                    let opacity = if i == 0 { "bg-muted" } else { "bg-muted/50" };
                    view! { <div class=format!("h-8 {} rounded-md", opacity)></div> }
                })
                .collect_view()}
        </div>
    }
}

#[component]
pub fn EmptyState(
    #[prop(into)] icon: AnyView,
    #[prop(into)] title: String,
    #[prop(optional, into)] description: String,
    #[prop(optional, into)] class: String,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    view! {
        <div class=format!("flex flex-col items-center justify-center py-16 px-4 {}", class)>
            <div class="text-muted-foreground/40 mb-4">{icon}</div>
            <h3 class="text-sm font-semibold text-foreground mb-1">{title}</h3>
            {(!description.is_empty())
                .then(|| {
                    view! {
                        <p class="text-xs text-muted-foreground max-w-xs text-center">
                            {description}
                        </p>
                    }
                })}
            {if let Some(children) = children {
                view! { <div class="mt-4">{children()}</div> }.into_any()
            } else {
                "".into_any()
            }}
        </div>
    }
}

#[component]
pub fn Tooltip(
    #[prop(into)] content: String,
    #[prop(optional, into)] class: String,
    children: Children,
) -> impl IntoView {
    view! {
        <div class=format!(
            "relative group inline-flex {}",
            class,
        )>
            {children()}
            <div class="absolute bottom-full left-1/2 -translate-x-1/2 mb-2 px-3 py-1.5 bg-popover border border-border rounded-md shadow-md text-xs text-popover-foreground whitespace-nowrap opacity-0 group-hover:opacity-100 transition-opacity pointer-events-none z-50">
                {content}
            </div>
        </div>
    }
}
