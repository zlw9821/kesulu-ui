use leptos::prelude::*;

#[component]
pub fn Separator(
    #[prop(optional, default = false)] vertical: bool,
    #[prop(optional, into)] class: String,
) -> impl IntoView {
    let orientation_class = if vertical {
        "h-full w-[1px]"
    } else {
        "h-[1px] w-full"
    };
    view! {
        <div
            role="separator"
            class=format!("shrink-0 bg-border {} {}", orientation_class, class)
        ></div>
    }
}

#[component]
pub fn PageHeader(
    #[prop(into)] title: String,
    #[prop(optional, into)] subtitle: String,
    #[prop(optional, into)] class: String,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    view! {
        <div class=format!("flex items-center justify-between {}", class)>
            <div>
                <h1 class="text-2xl font-bold tracking-tight text-foreground">{title}</h1>
                {(!subtitle.is_empty())
                    .then(|| {
                        view! { <p class="text-sm text-muted-foreground mt-1">{subtitle}</p> }
                    })}
            </div>
            <div class="flex items-center gap-3">
                {if let Some(children) = children { children().into_any() } else { "".into_any() }}
            </div>
        </div>
    }
}

/// Section header: an uppercase label with an optional right-aligned actions
/// slot (buttons, badges, links). Generic — the reusable form of the app's old
/// `SectionLabel`.
#[component]
pub fn SectionHeader(
    #[prop(into)] title: String,
    #[prop(optional)] actions: Option<ViewFnOnce>,
    #[prop(optional, into)] class: String,
) -> impl IntoView {
    view! {
        <div class=format!("flex items-center justify-between mb-2.5 {}", class)>
            <h3 class="text-[11px] font-semibold tracking-[0.06em] uppercase text-muted-foreground">
                {title}
            </h3>
            {actions.map(|a| a.run())}
        </div>
    }
}

#[component]
pub fn SectionTitle(
    #[prop(into)] title: String,
    #[prop(into)] subtitle: String,
    #[prop(optional, into)] class: String,
) -> impl IntoView {
    view! {
        <div class=format!("mb-5 {}", class)>
            <h3 class="text-xs font-semibold text-muted-foreground uppercase tracking-wide">
                {title}
            </h3>
            {(!subtitle.is_empty())
                .then(|| {
                    view! { <p class="text-xs text-muted-foreground/60 mt-0.5">{subtitle}</p> }
                })}
        </div>
    }
}

#[component]
pub fn ScrollArea(
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] max_height: String,
    children: Children,
) -> impl IntoView {
    let style = if max_height.is_empty() {
        String::new()
    } else {
        format!("max-height: {max_height};")
    };
    view! {
        <div
            class=format!(
                "overflow-y-auto scrollbar-thin scrollbar-thumb-border scrollbar-track-transparent {}",
                class,
            )
            style=style
        >
            {children()}
        </div>
    }
}
