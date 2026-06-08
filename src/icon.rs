//! Inlined lucide icon set — one primitive consolidating the SVG glyphs the
//! `ui` components need, with **no icon dependency** (lucide `<path>` data is
//! inlined). Add a variant to [`IconName`] when a component needs a new glyph:
//! copy the shapes from <https://lucide.dev> and keep lucide's kebab-case name.
//!
//! Every glyph shares lucide's 24×24 stroke geometry (`fill=none`,
//! `stroke=currentColor`, round caps/joins), so an [`Icon`] inherits its color
//! from the surrounding text (`currentColor`) and its size from the `class`
//! (default `size-4`). This is the project-blessed alternative to pulling in
//! `icondata`/`leptos_icons`: tiny, no extra deps, tree-shaken (only the glyphs
//! you name ship), and visually identical to shadcn's lucide-react defaults.

use leptos::prelude::*;

/// The set of inlined lucide glyphs. Names mirror lucide's own glyph names.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum IconName {
    /// lucide `chevron-down`
    ChevronDown,
    /// lucide `chevron-left`
    ChevronLeft,
    /// lucide `chevron-right`
    ChevronRight,
    /// lucide `check`
    Check,
    /// lucide `x`
    X,
    /// lucide `search`
    Search,
    /// lucide `ellipsis` (more-horizontal)
    Ellipsis,
    /// lucide `circle-alert`
    CircleAlert,
    /// lucide `layout-dashboard`
    LayoutDashboard,
    /// lucide `refresh-cw`
    RefreshCw,
    /// lucide `globe`
    Globe,
    /// lucide `trending-up`
    TrendingUp,
    /// lucide `settings`
    Settings,
}

impl IconName {
    /// The glyph's inner SVG shapes, wrapped in a `<g>` so the body is always a
    /// single element — this lets the match arms unify to one `AnyView`.
    fn body(self) -> AnyView {
        match self {
            Self::ChevronDown => view! {
                <g>
                    <path d="m6 9 6 6 6-6" />
                </g>
            }
            .into_any(),
            Self::ChevronLeft => view! {
                <g>
                    <path d="m15 18-6-6 6-6" />
                </g>
            }
            .into_any(),
            Self::ChevronRight => view! {
                <g>
                    <path d="m9 18 6-6-6-6" />
                </g>
            }
            .into_any(),
            Self::Check => view! {
                <g>
                    <polyline points="20 6 9 17 4 12" />
                </g>
            }
            .into_any(),
            Self::X => view! {
                <g>
                    <path d="M18 6 6 18" />
                    <path d="m6 6 12 12" />
                </g>
            }
            .into_any(),
            Self::Search => view! {
                <g>
                    <circle cx="11" cy="11" r="8" />
                    <path d="m21 21-4.3-4.3" />
                </g>
            }
            .into_any(),
            Self::Ellipsis => view! {
                <g>
                    <circle cx="12" cy="12" r="1" />
                    <circle cx="19" cy="12" r="1" />
                    <circle cx="5" cy="12" r="1" />
                </g>
            }
            .into_any(),
            Self::CircleAlert => view! {
                <g>
                    <circle cx="12" cy="12" r="10" />
                    <line x1="12" y1="8" x2="12" y2="12" />
                    <line x1="12" y1="16" x2="12.01" y2="16" />
                </g>
            }
            .into_any(),
            Self::LayoutDashboard => view! {
                <g>
                    <rect width="7" height="9" x="3" y="3" rx="1" />
                    <rect width="7" height="5" x="14" y="3" rx="1" />
                    <rect width="7" height="9" x="14" y="12" rx="1" />
                    <rect width="7" height="5" x="3" y="16" rx="1" />
                </g>
            }
            .into_any(),
            Self::RefreshCw => view! {
                <g>
                    <path d="M3 12a9 9 0 0 1 9-9 9.75 9.75 0 0 1 6.74 2.74L21 8" />
                    <path d="M21 3v5h-5" />
                    <path d="M21 12a9 9 0 0 1-9 9 9.75 9.75 0 0 1-6.74-2.74L3 16" />
                    <path d="M8 16H3v5" />
                </g>
            }
            .into_any(),
            Self::Globe => view! {
                <g>
                    <circle cx="12" cy="12" r="10" />
                    <path d="M12 2a14.5 14.5 0 0 0 0 20 14.5 14.5 0 0 0 0-20" />
                    <path d="M2 12h20" />
                </g>
            }
            .into_any(),
            Self::TrendingUp => view! {
                <g>
                    <path d="M16 7h6v6" />
                    <path d="m22 7-8.5 8.5-5-5L2 17" />
                </g>
            }
            .into_any(),
            Self::Settings => view! {
                <g>
                    <path d="M12.22 2h-.44a2 2 0 0 0-2 2v.18a2 2 0 0 1-1 1.73l-.43.25a2 2 0 0 1-2 0l-.15-.08a2 2 0 0 0-2.73.73l-.22.38a2 2 0 0 0 .73 2.73l.15.1a2 2 0 0 1 1 1.72v.51a2 2 0 0 1-1 1.74l-.15.09a2 2 0 0 0-.73 2.73l.22.38a2 2 0 0 0 2.73.73l.15-.08a2 2 0 0 1 2 0l.43.25a2 2 0 0 1 1 1.73V20a2 2 0 0 0 2 2h.44a2 2 0 0 0 2-2v-.18a2 2 0 0 1 1-1.73l.43-.25a2 2 0 0 1 2 0l.15.08a2 2 0 0 0 2.73-.73l.22-.39a2 2 0 0 0-.73-2.73l-.15-.08a2 2 0 0 1-1-1.74v-.5a2 2 0 0 1 1-1.74l.15-.09a2 2 0 0 0 .73-2.73l-.22-.38a2 2 0 0 0-2.73-.73l-.15.08a2 2 0 0 1-2 0l-.43-.25a2 2 0 0 1-1-1.73V4a2 2 0 0 0-2-2z" />
                    <circle cx="12" cy="12" r="3" />
                </g>
            }
            .into_any(),
        }
    }
}

/// Renders an inlined lucide [`IconName`] as a 24×24 stroke `<svg>`.
///
/// Color follows the surrounding text (`currentColor`); size comes from `class`
/// (default `size-4`). When non-empty, `class` *replaces* the default, so include
/// a size, e.g. `class="size-3.5 opacity-50"`.
#[component]
pub fn Icon(
    /// Which glyph to render.
    name: IconName,
    /// Utility classes for the `<svg>` (size/color/opacity). Replaces the
    /// `size-4` default when non-empty.
    #[prop(optional, into)]
    class: String,
    /// SVG stroke width (lucide default `2`; the checkbox uses `3` for a bolder
    /// check at small sizes).
    #[prop(optional, into)]
    stroke_width: String,
) -> impl IntoView {
    let class = if class.is_empty() {
        "size-4".to_string()
    } else {
        class
    };
    let stroke_width = if stroke_width.is_empty() {
        "2".to_string()
    } else {
        stroke_width
    };
    view! {
        <svg
            class=class
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width=stroke_width
            stroke-linecap="round"
            stroke-linejoin="round"
        >
            {name.body()}
        </svg>
    }
}
