use leptos::prelude::*;

#[component]
pub fn Card(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    view! {
        <div class=format!(
            "rounded-lg border border-border bg-card text-card-foreground shadow-sm {}",
            class,
        )>{children()}</div>
    }
}

#[component]
pub fn CardHeader(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    view! { <div class=format!("flex flex-col space-y-1.5 p-6 {}", class)>{children()}</div> }
}

#[component]
pub fn CardTitle(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    view! {
        <h3 class=format!(
            "text-lg font-semibold leading-none tracking-tight {}",
            class,
        )>{children()}</h3>
    }
}

#[component]
pub fn CardDescription(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    view! { <p class=format!("text-sm text-muted-foreground {}", class)>{children()}</p> }
}

#[component]
pub fn CardContent(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    view! { <div class=format!("p-6 pt-0 {}", class)>{children()}</div> }
}

#[component]
pub fn CardFooter(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    view! { <div class=format!("flex items-center p-6 pt-0 {}", class)>{children()}</div> }
}
