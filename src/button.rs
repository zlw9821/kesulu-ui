use leptos::prelude::*;

#[derive(Default, Clone, Copy, PartialEq)]
pub enum ButtonVariant {
    #[default]
    Default,
    Destructive,
    /// 危险但次要的逃生动作（如紧急平仓）：实心暗红底 + 白字——明确是个按钮、保留危险语义，
    /// 又以更暗的红比"主危险"（亮红 `Destructive`，用于真正的提交/确认）低一档，不抢主操作。
    DestructiveMuted,
    Outline,
    Secondary,
    Ghost,
    Link,
}

#[derive(Default, Clone, Copy, PartialEq)]
pub enum ButtonSize {
    Sm,
    #[default]
    Default,
    Lg,
    Icon,
}

#[component]
pub fn Button(
    #[prop(optional)] variant: ButtonVariant,
    #[prop(optional)] size: ButtonSize,
    #[prop(optional, into)] disabled: MaybeProp<bool>,
    #[prop(optional, into)] loading: MaybeProp<bool>,
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] on_click: Option<Callback<()>>,
    children: Children,
) -> impl IntoView {
    let variant_class = match variant {
        ButtonVariant::Default => {
            "bg-primary text-primary-foreground hover:bg-primary/90 shadow-sm"
        }
        ButtonVariant::Destructive => {
            "bg-destructive text-destructive-foreground hover:bg-destructive/90 shadow-sm"
        }
        ButtonVariant::DestructiveMuted => {
            "bg-destructive-muted text-destructive-foreground hover:bg-destructive shadow-sm"
        }
        ButtonVariant::Outline => {
            "border border-border-strong bg-transparent text-foreground hover:bg-muted hover:text-foreground shadow-sm"
        }
        ButtonVariant::Secondary => "bg-secondary text-secondary-foreground hover:bg-secondary/80",
        ButtonVariant::Ghost => "hover:bg-secondary hover:text-foreground",
        ButtonVariant::Link => "text-primary underline-offset-4 hover:underline",
    };
    let size_class = match size {
        ButtonSize::Sm => "h-8 px-3 text-xs rounded-md gap-1.5",
        ButtonSize::Default => "h-9 px-4 py-2 text-sm rounded-md gap-2",
        ButtonSize::Lg => "h-10 px-6 text-sm rounded-md gap-2",
        ButtonSize::Icon => "h-9 w-9 rounded-md",
    };
    let base = "inline-flex items-center justify-center font-medium transition-colors duration-200 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 focus-visible:ring-offset-background";
    let full_class = move || {
        let is_disabled = disabled.get().unwrap_or(false) || loading.get().unwrap_or(false);
        let state_class = if is_disabled {
            "opacity-50 pointer-events-none"
        } else {
            "active:scale-[0.97]"
        };
        format!("{base} {variant_class} {size_class} {state_class} {class}")
    };
    let is_loading = move || loading.get().unwrap_or(false);
    view! {
        <button
            class=full_class
            disabled=move || disabled.get().unwrap_or(false) || is_loading()
            on:click=move |_| {
                if let Some(cb) = &on_click {
                    cb.run(());
                }
            }
        >
            <Show when=is_loading>
                <span class="inline-block w-3.5 h-3.5 border-2 border-current border-t-transparent rounded-full animate-spin"></span>
            </Show>
            {children()}
        </button>
    }
}
