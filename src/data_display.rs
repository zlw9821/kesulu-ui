use leptos::children::ViewFn;
use leptos::prelude::*;

#[derive(Default, Clone, Copy)]
pub enum BadgeVariant {
    #[default]
    Neutral,
    Success,
    Danger,
    Warning,
    Info,
}

#[component]
pub fn Badge(
    #[prop(into)] text: String,
    #[prop(optional)] variant: BadgeVariant,
    #[prop(optional, into)] class: String,
) -> impl IntoView {
    let variant_class = match variant {
        BadgeVariant::Neutral => "border-border bg-secondary text-secondary-foreground",
        BadgeVariant::Success => "border-transparent bg-success-dim text-success",
        BadgeVariant::Danger => "border-transparent bg-destructive-dim text-destructive",
        BadgeVariant::Warning => "border-transparent bg-warning-dim text-warning",
        BadgeVariant::Info => "border-transparent bg-info-dim text-info",
    };
    view! {
        <span class=format!(
            "inline-flex items-center rounded-md border px-2 py-0.5 text-[10px] font-semibold transition-colors {} {}",
            variant_class,
            class,
        )>{text}</span>
    }
}

#[component]
pub fn TableHead(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    view! {
        <thead>
            <tr class=format!(
                "text-[10px] text-muted-foreground uppercase tracking-wider font-semibold border-b border-border {}",
                class,
            )>{children()}</tr>
        </thead>
    }
}

#[component]
pub fn DataTable(
    #[prop(into)] columns: Vec<Column>,
    #[prop(optional, into)] class: String,
    children: ChildrenFn,
) -> impl IntoView {
    view! {
        <div class=format!("overflow-x-auto {}", class)>
            <table class="w-full text-xs">
                <thead>
                    <tr class="text-[10px] text-muted-foreground uppercase tracking-wider font-bold border-b border-border">
                        {columns
                            .into_iter()
                            .map(|col| {
                                let align = match col.align {
                                    ColumnAlign::Left => "text-left",
                                    ColumnAlign::Right => "text-right",
                                    ColumnAlign::Center => "text-center",
                                };
                                view! { <th class=format!("{} py-2 px-3", align)>{col.label}</th> }
                            })
                            .collect_view()}
                    </tr>
                </thead>
                <tbody>{children()}</tbody>
            </table>
        </div>
    }
}

#[derive(Clone)]
pub struct Column {
    pub label: &'static str,
    pub align: ColumnAlign,
}

impl Column {
    pub fn new(label: &'static str) -> Self {
        Self {
            label,
            align: ColumnAlign::Left,
        }
    }

    pub fn right(label: &'static str) -> Self {
        Self {
            label,
            align: ColumnAlign::Right,
        }
    }

    pub fn center(label: &'static str) -> Self {
        Self {
            label,
            align: ColumnAlign::Center,
        }
    }
}

#[derive(Clone, Copy, Default)]
pub enum ColumnAlign {
    #[default]
    Left,
    Right,
    Center,
}

#[component]
pub fn KPICard(
    #[prop(into)] label: String,
    #[prop(into)] value: AnyView,
    #[prop(into)] sub_label: AnyView,
    #[prop(into, optional)] value_class: String,
    #[prop(optional)] compact: bool,
) -> impl IntoView {
    let container_class = if compact {
        "p-3 h-20 flex flex-col justify-between"
    } else {
        "p-5 hover:shadow-md hover:border-border-hover transition-all duration-200"
    };

    view! {
        <div class=format!("rounded-lg border border-border bg-card shadow-sm {}", container_class)>
            <div class="text-[10px] text-muted-foreground font-semibold uppercase tracking-wide mb-1">
                {label}
            </div>
            <div>
                <div class=format!("text-xl font-bold tracking-tight {}", value_class)>{value}</div>
                <div class="text-[10px] text-muted-foreground/60 font-medium mt-1 truncate">
                    {sub_label}
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn CurrencyDisplay(
    #[prop(into)] currency: String,
    #[prop(into)] value: f64,
    #[prop(optional, default = 2)] precision: usize,
    #[prop(optional, into)] class: String,
    #[prop(optional)] show_sign: bool,
) -> impl IntoView {
    let is_negative = value < 0.0;
    let abs_value = value.abs();
    let full_string = format!("{:.1$}", abs_value, precision);

    let (integer_part, decimal_part) = if precision > 0 {
        match full_string.split_once('.') {
            Some((i, d)) => (i.to_string(), format!(".{}", d)),
            None => (full_string, String::new()),
        }
    } else {
        (full_string, String::new())
    };

    view! {
        <span class=format!(
            "inline-flex items-baseline font-mono {}",
            class,
        )>
            {if is_negative {
                view! { <span class="mr-0.5">"-"</span> }.into_any()
            } else if show_sign && value > 0.0 {
                view! { <span class="mr-0.5">"+"</span> }.into_any()
            } else {
                "".into_any()
            }}
            <span class="text-[0.65em] font-bold opacity-50 mr-1.5 uppercase tracking-wider">
                {currency}
            </span> <span class="font-bold">{integer_part}</span>
            <span class="text-[0.75em] opacity-70 font-medium">{decimal_part}</span>
        </span>
    }
}

#[component]
pub fn ExpandableRow(#[prop(into)] cells: Vec<AnyView>, expanded_content: ViewFn) -> impl IntoView {
    let (expanded, set_expanded) = signal(false);
    view! {
        <tr
            class="border-b border-border hover:bg-muted/50 cursor-pointer transition-colors"
            on:click=move |_| set_expanded.update(|v| *v = !*v)
        >
            {cells
                .into_iter()
                .map(|cell| view! { <td class="py-2 px-3">{cell}</td> })
                .collect_view()}
            <td class="py-2 px-3 w-6 text-muted-foreground">
                <span class=move || {
                    format!(
                        "inline-block transition-transform duration-200 text-[10px] {}",
                        if expanded.get() { "rotate-90" } else { "" },
                    )
                }>"▶"</span>
            </td>
        </tr>
        <Show when=move || expanded.get()>
            <tr class="border-b border-border bg-muted/30 animate-fade-in">
                <td colspan="99" class="py-3 px-6 text-xs text-muted-foreground">
                    {expanded_content.run()}
                </td>
            </tr>
        </Show>
    }
}

#[component]
pub fn Progress(
    #[prop(into)] value: Signal<f64>,
    #[prop(optional, into)] class: String,
) -> impl IntoView {
    view! {
        <div class=format!("relative h-2 w-full overflow-hidden rounded-full bg-muted {}", class)>
            <div
                class="h-full bg-primary transition-all duration-300"
                style=move || format!("width: {}%", value.get().clamp(0.0, 100.0))
            ></div>
        </div>
    }
}
