use leptos::prelude::*;

#[component]
pub fn Spinner(#[prop(optional, into)] class: String) -> impl IntoView {
    view! {
        <svg
            role="status"
            aria-label="Loading"
            class=format!("size-4 animate-spin {class}")
            xmlns="http://www.w3.org/2000/svg"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
        >
            <path d="M21 12a9 9 0 1 1-6.219-8.56" />
        </svg>
    }
}
