use leptos::prelude::*;

#[derive(Default, Clone, Copy, PartialEq)]
pub enum AlertVariant {
    #[default]
    Default,
    Destructive,
    Warning,
}

impl AlertVariant {
    fn classes(self) -> &'static str {
        match self {
            Self::Default => "bg-card border-border text-foreground",
            Self::Destructive => "border-destructive/50 bg-destructive/10 text-destructive",
            Self::Warning => "border-warning/50 bg-warning/10 text-warning",
        }
    }
}

#[component]
pub fn Alert(
    #[prop(optional)] variant: AlertVariant,
    #[prop(optional, into)] class: String,
    children: Children,
) -> impl IntoView {
    view! {
        <div
            role="alert"
            class=format!(
                "relative w-full rounded-lg border px-4 py-3 text-sm {} {}",
                variant.classes(),
                class,
            )
        >
            {children()}
        </div>
    }
}

#[component]
pub fn AlertTitle(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    view! {
        <h5 class=format!(
            "mb-1 font-medium leading-none tracking-tight {}",
            class,
        )>{children()}</h5>
    }
}

#[component]
pub fn AlertDescription(
    #[prop(optional, into)] class: String,
    children: Children,
) -> impl IntoView {
    view! { <div class=format!("text-sm opacity-90 {}", class)>{children()}</div> }
}
