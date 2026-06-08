use leptos::prelude::*;

use super::icon::{Icon, IconName};
use super::presence::use_presence;

/// One selectable option: `value` is the stable key emitted on select, `label`
/// is the shown (and searched) text.
#[derive(Clone, PartialEq)]
pub struct ComboboxOption {
    pub value: String,
    pub label: String,
}

impl ComboboxOption {
    pub fn new(value: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            label: label.into(),
        }
    }
}

/// A searchable single-select: a trigger button plus a popover holding a filter
/// input and a substring-filtered option list. Pure (no web-sys, no JS lib) — the
/// "fuzzy" match is a Rust case-insensitive `contains`. Reuses `Select`'s
/// open / `use_presence` / click-outside-backdrop pattern.
///
/// `value`/`on_change` are the controlled selection (the option `value`). Sizing
/// is caller-driven via `class` on the root (e.g. `class="w-72"`).
#[component]
pub fn Combobox(
    #[prop(into)] value: Signal<String>,
    #[prop(into)] on_change: Callback<String>,
    #[prop(into)] options: Vec<ComboboxOption>,
    #[prop(optional, into)] placeholder: String,
    #[prop(optional, into)] search_placeholder: String,
    #[prop(optional, into)] empty_text: String,
    #[prop(optional, into)] class: String,
) -> impl IntoView {
    let open = RwSignal::new(false);
    let query = RwSignal::new(String::new());
    let p = use_presence(open.into(), 150);

    let options = StoredValue::new(options);
    let placeholder = StoredValue::new(if placeholder.is_empty() {
        "Select…".to_string()
    } else {
        placeholder
    });
    let search_placeholder = StoredValue::new(if search_placeholder.is_empty() {
        "Search…".to_string()
    } else {
        search_placeholder
    });
    let empty_text = StoredValue::new(if empty_text.is_empty() {
        "No results found.".to_string()
    } else {
        empty_text
    });

    let filtered = move || {
        let q = query.get().to_lowercase();
        options.with_value(|opts| {
            opts.iter()
                .filter(|o| q.is_empty() || o.label.to_lowercase().contains(&q))
                .cloned()
                .collect::<Vec<_>>()
        })
    };

    view! {
        <div class=format!("relative {}", class)>
            <button
                type="button"
                aria-expanded=move || open.get().to_string()
                on:click=move |_| open.update(|v| *v = !*v)
                class="flex h-9 w-full items-center justify-between gap-2 rounded-md border border-input bg-transparent px-3 text-sm whitespace-nowrap shadow-xs transition-colors outline-none focus-visible:border-ring focus-visible:ring-[3px] focus-visible:ring-ring/50"
            >
                <span class=move || {
                    let has = options.with_value(|o| o.iter().any(|x| x.value == value.get()));
                    if has { "line-clamp-1" } else { "line-clamp-1 text-muted-foreground" }
                }>
                    {move || {
                        let v = value.get();
                        options
                            .with_value(|o| {
                                o.iter().find(|x| x.value == v).map(|x| x.label.clone())
                            })
                            .unwrap_or_else(|| placeholder.get_value())
                    }}
                </span>
                <Icon name=IconName::ChevronDown class="size-4 shrink-0 opacity-50" />
            </button>
            <Show when=move || p.mounted.get()>
                // Click-outside-to-close backdrop (see select.rs / dropdown.rs).
                <div class="fixed inset-0 z-40" on:click=move |_| open.set(false)></div>
                <div class=move || {
                    format!(
                        "absolute left-0 top-full z-50 mt-1 w-full overflow-hidden rounded-md border border-border bg-popover text-popover-foreground shadow-md duration-150 {}",
                        if p.open.get() {
                            "animate-in fade-in-0 zoom-in-95 slide-in-from-top-2"
                        } else {
                            "animate-out fade-out-0 zoom-out-95 slide-out-to-top-2"
                        },
                    )
                }>
                    <div class="flex h-9 items-center gap-2 border-b border-border px-3">
                        <Icon name=IconName::Search class="size-4 shrink-0 opacity-50" />
                        <input
                            type="text"
                            value=move || query.get()
                            prop:value=move || query.get()
                            placeholder=move || search_placeholder.get_value()
                            class="flex h-9 w-full bg-transparent text-sm outline-none placeholder:text-muted-foreground"
                            on:input=move |ev| query.set(event_target_value(&ev))
                        />
                    </div>
                    <div class="max-h-[300px] overflow-x-hidden overflow-y-auto p-1">
                        {move || {
                            let items = filtered();
                            if items.is_empty() {
                                view! {
                                    <div class="py-6 text-center text-sm text-muted-foreground">
                                        {empty_text.get_value()}
                                    </div>
                                }
                                    .into_any()
                            } else {
                                items
                                    .into_iter()
                                    .map(|opt| {
                                        let selected = Memo::new({
                                            let v = opt.value.clone();
                                            move |_| value.get() == v
                                        });
                                        let val_click = opt.value.clone();
                                        let label = opt.label.clone();
                                        view! {
                                            <div
                                                role="option"
                                                aria-selected=move || selected.get().to_string()
                                                on:click=move |_| {
                                                    on_change.run(val_click.clone());
                                                    open.set(false);
                                                    query.set(String::new());
                                                }
                                                class="relative flex w-full cursor-default items-center gap-2 rounded-sm py-1.5 pr-8 pl-2 text-sm outline-hidden select-none hover:bg-accent hover:text-accent-foreground"
                                            >
                                                <span class="absolute right-2 flex size-3.5 items-center justify-center">
                                                    <Show when=move || selected.get()>
                                                        <Icon name=IconName::Check />
                                                    </Show>
                                                </span>
                                                {label}
                                            </div>
                                        }
                                    })
                                    .collect_view()
                                    .into_any()
                            }
                        }}
                    </div>
                </div>
            </Show>
        </div>
    }
}
