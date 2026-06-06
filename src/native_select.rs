use leptos::prelude::*;

/// Size variant for [`NativeSelect`] (shadcn `data-[size=*]`).
#[derive(Default, Clone, Copy, PartialEq)]
pub enum NativeSelectSize {
    #[default]
    Default,
    Sm,
}

impl NativeSelectSize {
    fn classes(self) -> &'static str {
        match self {
            NativeSelectSize::Default => "h-9 py-2",
            NativeSelectSize::Sm => "h-8 py-1",
        }
    }
}

/// A styled native `<select>` with a chevron overlay. Signal-driven like
/// [`crate::SelectInput`]; pass the `<option>`s as children.
#[component]
pub fn NativeSelect(
    #[prop(into)] value: Signal<String>,
    #[prop(into)] on_change: Callback<String>,
    #[prop(optional)] size: NativeSelectSize,
    #[prop(optional, into)] disabled: MaybeProp<bool>,
    #[prop(optional, into)] class: String,
    children: Children,
) -> impl IntoView {
    let is_disabled = move || disabled.get().unwrap_or(false);
    view! {
        <div class="group/native-select relative w-fit has-[select:disabled]:opacity-50">
            <select
                prop:value=value
                disabled=is_disabled
                class=format!(
                    "w-full min-w-0 appearance-none rounded-md border border-input bg-transparent px-3 pr-9 text-sm shadow-xs transition-[color,box-shadow] outline-none selection:bg-primary selection:text-primary-foreground placeholder:text-muted-foreground disabled:pointer-events-none disabled:cursor-not-allowed focus-visible:border-ring focus-visible:ring-[3px] focus-visible:ring-ring/50 aria-invalid:border-destructive aria-invalid:ring-destructive/20 {} {}",
                    size.classes(),
                    class,
                )
                on:change=move |ev| {
                    on_change.run(event_target_value(&ev));
                }
            >
                {children()}
            </select>
            <svg
                class="pointer-events-none absolute top-1/2 right-3.5 size-4 -translate-y-1/2 text-muted-foreground opacity-50 select-none"
                aria-hidden="true"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                stroke-width="2"
                stroke-linecap="round"
                stroke-linejoin="round"
            >
                <path d="m6 9 6 6 6-6" />
            </svg>
        </div>
    }
}

/// A styled `<option>` for use inside [`NativeSelect`].
#[component]
pub fn NativeSelectOption(
    #[prop(optional, into)] value: String,
    #[prop(optional, into)] disabled: bool,
    #[prop(optional, into)] class: String,
    children: Children,
) -> impl IntoView {
    view! {
        <option
            value=value
            disabled=disabled
            class=format!("bg-[Canvas] text-[CanvasText] {}", class)
        >
            {children()}
        </option>
    }
}

/// A styled `<optgroup>` for use inside [`NativeSelect`].
#[component]
pub fn NativeSelectOptGroup(
    #[prop(optional, into)] label: String,
    #[prop(optional, into)] class: String,
    children: Children,
) -> impl IntoView {
    view! {
        <optgroup label=label class=format!("bg-[Canvas] text-[CanvasText] {}", class)>
            {children()}
        </optgroup>
    }
}
