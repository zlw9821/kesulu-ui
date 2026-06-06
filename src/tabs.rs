use leptos::prelude::*;
use leptos_router::components::A;
use leptos_router::hooks::use_location;

/// Declarative description of a route-driven tab.
#[derive(Clone, Copy)]
pub struct RouteTab {
    /// Target href (also used as the active-match base).
    pub href: &'static str,
    /// Display label.
    pub label: &'static str,
    /// Extra prefixes that should also count as active (e.g. /performance/attribution → /performance/sectors).
    pub aliases: &'static [&'static str],
    /// `true` means active only on exact match (use for "/" / index tabs).
    pub exact: bool,
}

impl RouteTab {
    pub const fn new(href: &'static str, label: &'static str) -> Self {
        Self {
            href,
            label,
            aliases: &[],
            exact: false,
        }
    }

    pub const fn exact(href: &'static str, label: &'static str) -> Self {
        Self {
            href,
            label,
            aliases: &[],
            exact: true,
        }
    }

    pub const fn with_aliases(mut self, aliases: &'static [&'static str]) -> Self {
        self.aliases = aliases;
        self
    }
}

/// Router-aware tab strip used by Trading / Portfolio / Performance shells.
/// Replaces the per-shell hand-written `tab_class` closures with one source of truth.
#[component]
pub fn RouteTabs(tabs: Vec<RouteTab>) -> impl IntoView {
    let location = use_location();
    view! {
        <div class="flex gap-1 flex-wrap">
            {tabs
                .into_iter()
                .map(|tab| {
                    let RouteTab { href, label, aliases, exact } = tab;
                    let active = move || {
                        let path = location.pathname.get();
                        let base = if exact {
                            path == href || path == format!("{}/", href)
                        } else {
                            path.starts_with(href)
                        };
                        base || aliases.iter().any(|a| path.starts_with(a))
                    };
                    view! {
                        <A
                            href=href
                            attr:class=move || {
                                if active() {
                                    "px-3 py-1.5 rounded-md text-xs font-bold bg-foreground text-background"
                                } else {
                                    "px-3 py-1.5 rounded-md text-xs font-bold text-muted-foreground hover:bg-secondary transition-colors"
                                }
                            }
                        >
                            {label}
                        </A>
                    }
                })
                .collect_view()}
        </div>
    }
}

/// Visual style for [`Tabs`]. `Pill` is the shadcn default (muted segmented
/// container); `Underline` is a flat underline strip that reads better on dense,
/// dark surfaces where the muted pill nearly vanishes.
#[derive(Default, Clone, Copy, PartialEq)]
pub enum TabsVariant {
    #[default]
    Pill,
    Underline,
}

#[component]
pub fn Tabs(
    #[prop(into)] value: RwSignal<&'static str>,
    #[prop(optional)] variant: TabsVariant,
    #[prop(optional, into)] class: String,
    children: Children,
) -> impl IntoView {
    provide_context(value);
    provide_context(variant);
    view! { <div class=format!("flex flex-col {}", class)>{children()}</div> }
}

#[component]
pub fn TabsList(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    let variant = use_context::<TabsVariant>().unwrap_or_default();
    let base = match variant {
        TabsVariant::Pill => {
            "inline-flex h-9 items-center justify-start rounded-lg bg-muted p-1 text-muted-foreground"
        }
        TabsVariant::Underline => "flex items-center gap-1 border-b border-border",
    };
    view! { <div class=format!("{base} {}", class)>{children()}</div> }
}

#[component]
pub fn TabsTrigger(
    #[prop(into)] value: &'static str,
    #[prop(optional, into)] class: String,
    children: Children,
) -> impl IntoView {
    let active_tab = expect_context::<RwSignal<&'static str>>();
    let variant = use_context::<TabsVariant>().unwrap_or_default();
    let is_active = move || active_tab.get() == value;
    view! {
        <button
            on:click=move |_| active_tab.set(value)
            class=move || {
                let (base, active, inactive) = match variant {
                    TabsVariant::Pill => {
                        (
                            "inline-flex items-center justify-center whitespace-nowrap rounded-md px-3 py-1 text-sm font-medium transition-all duration-200 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring",
                            "bg-background text-foreground shadow-sm",
                            "hover:text-foreground/80",
                        )
                    }
                    TabsVariant::Underline => {
                        (
                            "inline-flex items-center justify-center whitespace-nowrap -mb-px border-b-2 px-3.5 py-2 text-sm transition-colors focus-visible:outline-none",
                            "border-accent text-foreground font-semibold",
                            "border-transparent text-muted-foreground font-medium hover:text-foreground/80",
                        )
                    }
                };
                format!("{base} {} {}", if is_active() { active } else { inactive }, class)
            }
        >
            {children()}
        </button>
    }
}

#[component]
pub fn TabsContent(
    #[prop(into)] value: &'static str,
    #[prop(optional, into)] class: String,
    children: ChildrenFn,
) -> impl IntoView {
    let active_tab = expect_context::<RwSignal<&'static str>>();
    let visited = RwSignal::new(active_tab.get_untracked() == value);
    Effect::new(move |_| {
        if active_tab.get() == value {
            visited.set(true);
        }
    });
    view! {
        <Show when=move || visited.get()>
            <div
                class=format!(
                    "mt-4 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring {}",
                    class,
                )
                style=move || if active_tab.get() == value { "" } else { "display:none" }
            >
                {children()}
            </div>
        </Show>
    }
}
