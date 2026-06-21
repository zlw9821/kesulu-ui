use leptos::html::Textarea;
use leptos::prelude::*;

use super::icon::{Icon, IconName};

#[component]
pub fn TextInput(
    #[prop(into)] value: Signal<String>,
    #[prop(into)] on_input: Callback<String>,
    #[prop(optional, into)] placeholder: String,
    #[prop(optional, into)] label: String,
    #[prop(optional, into)] input_type: String,
) -> impl IntoView {
    let input_type = if input_type.is_empty() {
        "text".to_string()
    } else {
        input_type
    };
    view! {
        <div class="flex flex-col gap-1.5">
            {(!label.is_empty())
                .then(|| {
                    view! {
                        <label class="text-[10px] font-medium text-muted-foreground uppercase tracking-wide">
                            {label.clone()}
                        </label>
                    }
                })}
            <input
                type=input_type
                prop:value=value
                placeholder=placeholder
                class="flex h-9 w-full rounded-md border border-input bg-background px-3 py-1 text-sm text-foreground shadow-sm transition-colors placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring"
                on:input=move |ev| {
                    on_input.run(event_target_value(&ev));
                }
            />
        </div>
    }
}

#[component]
pub fn NumberInput(
    value: Signal<f64>,
    on_input: Callback<f64>,
    #[prop(optional)] step: f64,
    #[prop(optional, into)] label: String,
    #[prop(optional, into)] unit: String,
    #[prop(optional, into)] helper: String,
    #[prop(optional)] min: Option<f64>,
    #[prop(optional)] max: Option<f64>,
) -> impl IntoView {
    let _ = step;
    let buffer = RwSignal::new(number_input_format(value.get_untracked()));

    Effect::new(move |_| {
        let v = value.get();
        if buffer.with_untracked(|b| b.parse::<f64>().ok() != Some(v)) {
            buffer.set(number_input_format(v));
        }
    });

    view! {
        <div class="flex flex-col gap-1.5">
            {(!label.is_empty())
                .then(|| {
                    view! {
                        <label class="text-[10px] font-medium text-muted-foreground uppercase tracking-wide">
                            {label.clone()}
                        </label>
                    }
                })} <div class="relative">
                <input
                    type="text"
                    inputmode="decimal"
                    // Bind the `value` attribute (SSR-rendered + reactive) rather than
                    // `prop:value`: a client-only property does not hydrate inside
                    // streamed `<Suspense>` content, leaving the field blank on
                    // resource-backed pages. `prop:value` keeps it controlled on top.
                    value=move || buffer.get()
                    prop:value=move || buffer.get()
                    class="flex h-9 w-full rounded-md border border-input bg-background px-3 py-1 text-sm text-foreground shadow-sm transition-colors placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring"
                    on:input=move |ev| {
                        let text = event_target_value(&ev);
                        buffer.set(text.clone());
                        if let Ok(v) = text.parse::<f64>() {
                            on_input.run(number_input_clamp(v, min, max));
                        }
                    }
                    on:blur=move |_| {
                        match buffer.get().parse::<f64>() {
                            Ok(v) => {
                                let clamped = number_input_clamp(v, min, max);
                                on_input.run(clamped);
                                buffer.set(number_input_format(clamped));
                            }
                            Err(_) => buffer.set(number_input_format(value.get_untracked())),
                        }
                    }
                />
                {(!unit.is_empty())
                    .then(|| {
                        view! {
                            <span class="absolute right-3 top-1/2 -translate-y-1/2 text-[10px] font-medium text-muted-foreground uppercase">
                                {unit.clone()}
                            </span>
                        }
                    })}
            </div>
            {(!helper.is_empty())
                .then(|| {
                    view! {
                        <span class="text-[10px] text-muted-foreground">{helper.clone()}</span>
                    }
                })}
        </div>
    }
}

fn number_input_format(v: f64) -> String {
    if v.fract() == 0.0 && v.abs() < 1e15 {
        format!("{v:.0}")
    } else {
        v.to_string()
    }
}

fn number_input_clamp(v: f64, min: Option<f64>, max: Option<f64>) -> f64 {
    let mut out = v;
    if let Some(lo) = min {
        out = out.max(lo);
    }
    if let Some(hi) = max {
        out = out.min(hi);
    }
    out
}

#[component]
pub fn SelectInput(
    #[prop(into)] value: Signal<String>,
    #[prop(into)] on_change: Callback<String>,
    #[prop(into)] options: Vec<(String, String)>,
    #[prop(optional, into)] label: String,
) -> impl IntoView {
    view! {
        <div class="flex flex-col gap-1.5">
            {(!label.is_empty())
                .then(|| {
                    view! {
                        <label class="text-[10px] font-medium text-muted-foreground uppercase tracking-wide">
                            {label.clone()}
                        </label>
                    }
                })}
            <select
                prop:value=value
                class="flex h-9 w-full rounded-md border border-input bg-background px-3 py-1 text-sm text-foreground shadow-sm transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring"
                on:change=move |ev| {
                    on_change.run(event_target_value(&ev));
                }
            >
                {options
                    .into_iter()
                    .map(|(val, display)| {
                        view! { <option value=val.clone()>{display}</option> }
                    })
                    .collect_view()}
            </select>
        </div>
    }
}

#[component]
pub fn Label(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    view! {
        <label class=format!(
            "text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70 {}",
            class,
        )>{children()}</label>
    }
}

#[component]
pub fn Switch(
    #[prop(into)] checked: Signal<bool>,
    #[prop(into)] on_change: Callback<bool>,
    #[prop(optional, into)] disabled: MaybeProp<bool>,
    #[prop(optional, into)] class: String,
) -> impl IntoView {
    let is_disabled = move || disabled.get().unwrap_or(false);
    view! {
        <button
            role="switch"
            type="button"
            aria-checked=move || checked.get().to_string()
            disabled=is_disabled
            on:click=move |_| {
                if !is_disabled() {
                    on_change.run(!checked.get());
                }
            }
            class=move || {
                format!(
                    "peer inline-flex h-5 w-9 shrink-0 cursor-pointer items-center rounded-full border-2 border-transparent transition-colors duration-200 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 focus-visible:ring-offset-background disabled:cursor-not-allowed disabled:opacity-50 {} {}",
                    if checked.get() { "bg-primary" } else { "bg-input" },
                    class,
                )
            }
        >
            <span class=move || {
                format!(
                    "pointer-events-none block h-4 w-4 rounded-full bg-background shadow-lg ring-0 transition-transform duration-200 {}",
                    if checked.get() { "translate-x-4" } else { "translate-x-0" },
                )
            }></span>
        </button>
    }
}

#[component]
pub fn TextArea(
    #[prop(into)] value: Signal<String>,
    #[prop(into)] on_input: Callback<String>,
    #[prop(optional, into)] placeholder: String,
    #[prop(optional, into)] label: String,
    #[prop(optional, into)] class: String,
    #[prop(optional, default = 3)] rows: u32,
    #[prop(optional)] auto_resize: bool,
    #[prop(optional, into)] disabled: MaybeProp<bool>,
    #[prop(optional, into)] node_ref: Option<NodeRef<Textarea>>,
) -> impl IntoView {
    let internal_ref = NodeRef::<Textarea>::new();
    let ta_ref = node_ref.unwrap_or(internal_ref);

    let resize_class = if auto_resize {
        "resize-none overflow-y-auto"
    } else {
        "resize-y"
    };

    view! {
        <div class="flex flex-col gap-1.5">
            {(!label.is_empty())
                .then(|| {
                    view! {
                        <label class="text-[10px] font-medium text-muted-foreground uppercase tracking-wide">
                            {label.clone()}
                        </label>
                    }
                })}
            <textarea
                node_ref=ta_ref
                rows=rows
                placeholder=placeholder
                disabled=move || disabled.get().unwrap_or(false)
                prop:value=value
                class=format!(
                    "flex w-full rounded-md border border-input bg-background px-3 py-2 text-sm text-foreground shadow-sm transition-colors placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring disabled:cursor-not-allowed disabled:opacity-50 {} {}",
                    resize_class,
                    class,
                )
                on:input=move |ev| {
                    let val = event_target_value(&ev);
                    on_input.run(val);
                    if auto_resize && let Some(ta) = ta_ref.get() {
                        ta.style(("height", "auto"));
                        let scroll_h = ta.scroll_height();
                        ta.style(("height", format!("{}px", scroll_h)));
                    }
                }
            />
        </div>
    }
}

#[component]
pub fn Checkbox(
    #[prop(into)] checked: Signal<bool>,
    #[prop(into)] on_change: Callback<bool>,
    #[prop(optional, into)] disabled: MaybeProp<bool>,
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] label: String,
) -> impl IntoView {
    let is_disabled = move || disabled.get().unwrap_or(false);
    view! {
        <label class=format!(
            "inline-flex items-center gap-2 cursor-pointer select-none {}",
            if is_disabled() { "opacity-50 cursor-not-allowed" } else { "" },
        )>
            <button
                type="button"
                role="checkbox"
                aria-checked=move || checked.get().to_string()
                disabled=is_disabled
                on:click=move |ev| {
                    ev.stop_propagation();
                    if !is_disabled() {
                        on_change.run(!checked.get());
                    }
                }
                class=move || {
                    format!(
                        "h-4 w-4 shrink-0 rounded-sm border border-input ring-offset-background transition-colors duration-150 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed {} {}",
                        if checked.get() {
                            "bg-primary border-primary text-primary-foreground"
                        } else {
                            "bg-background"
                        },
                        class,
                    )
                }
            >
                <Show when=move || checked.get()>
                    <Icon name=IconName::Check class="h-3 w-3 mx-auto" stroke_width="3" />
                </Show>
            </button>
            {(!label.is_empty())
                .then(|| {
                    view! { <span class="text-sm font-medium leading-none">{label}</span> }
                })}
        </label>
    }
}
