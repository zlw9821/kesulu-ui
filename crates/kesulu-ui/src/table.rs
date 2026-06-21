use leptos::prelude::*;

/// Presentational table primitives — a faithful shadcn/ui port. These are pure
/// layout/style wrappers around the native HTML table elements; data-grid
/// behaviour (sort/filter/paginate) is app-level composition, not part of `ui`.

#[component]
pub fn Table(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    view! {
        <div class="relative w-full overflow-x-auto">
            <table class=format!("w-full caption-bottom text-sm {}", class)>{children()}</table>
        </div>
    }
}

#[component]
pub fn TableHeader(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    view! { <thead class=format!("[&_tr]:border-b {}", class)>{children()}</thead> }
}

#[component]
pub fn TableBody(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    view! { <tbody class=format!("[&_tr:last-child]:border-0 {}", class)>{children()}</tbody> }
}

#[component]
pub fn TableFooter(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    view! {
        <tfoot class=format!(
            "border-t border-border bg-muted/50 font-medium [&>tr]:last:border-b-0 {}",
            class,
        )>{children()}</tfoot>
    }
}

#[component]
pub fn TableRow(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    view! {
        <tr class=format!(
            "border-b border-border transition-colors hover:bg-muted/50 has-aria-expanded:bg-muted/50 data-[state=selected]:bg-muted {}",
            class,
        )>{children()}</tr>
    }
}

#[component]
pub fn TableHead(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    view! {
        <th class=format!(
            "h-10 px-2 text-left align-middle font-medium whitespace-nowrap text-foreground [&:has([role=checkbox])]:pr-0 [&>[role=checkbox]]:translate-y-[2px] {}",
            class,
        )>{children()}</th>
    }
}

#[component]
pub fn TableCell(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    view! {
        <td class=format!(
            "p-2 align-middle whitespace-nowrap [&:has([role=checkbox])]:pr-0 [&>[role=checkbox]]:translate-y-[2px] {}",
            class,
        )>{children()}</td>
    }
}

#[component]
pub fn TableCaption(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    view! {
        <caption class=format!(
            "mt-4 text-sm text-muted-foreground {}",
            class,
        )>{children()}</caption>
    }
}
