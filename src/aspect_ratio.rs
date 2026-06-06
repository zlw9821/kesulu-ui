use leptos::prelude::*;

/// Constrains its children to a fixed width/height ratio.
///
/// shadcn's `AspectRatio` is a thin wrapper over Radix's primitive, which sets
/// the CSS `aspect-ratio` property. We express that directly via an inline
/// `style` on a relative wrapper — no Radix runtime needed.
#[component]
pub fn AspectRatio(
    /// Width-to-height ratio (e.g. `16.0 / 9.0`). Defaults to a square (`1.0`).
    #[prop(optional, default = 1.0)]
    ratio: f64,
    #[prop(optional, into)] class: String,
    children: Children,
) -> impl IntoView {
    view! {
        <div class=format!("relative w-full {}", class) style=format!("aspect-ratio: {ratio};")>
            {children()}
        </div>
    }
}
