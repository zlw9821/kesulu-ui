use leptos::prelude::*;

#[derive(Debug, Clone)]
pub struct Toast {
    pub id: u64,
    pub message: String,
    pub level: ToastLevel,
    pub persistent: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToastLevel {
    Success,
    Error,
    Warning,
    Info,
}

/// Toast queue. Provide one via `provide_context(Toaster::new())` near the root,
/// mount `<ToastContainer/>` once, then call `.success(..)` / `.error(..)` etc.
/// from anywhere with `use_context::<Toaster>()`.
#[derive(Clone, Copy)]
pub struct Toaster {
    toasts: RwSignal<Vec<Toast>>,
    counter: RwSignal<u64>,
}

impl Default for Toaster {
    fn default() -> Self {
        Self::new()
    }
}

impl Toaster {
    pub fn new() -> Self {
        Self {
            toasts: RwSignal::new(Vec::new()),
            counter: RwSignal::new(0),
        }
    }

    pub fn push(&self, message: impl Into<String>, level: ToastLevel) {
        self.push_opts(message, level, false);
    }

    pub fn push_persistent(&self, message: impl Into<String>, level: ToastLevel) {
        self.push_opts(message, level, true);
    }

    fn push_opts(&self, message: impl Into<String>, level: ToastLevel, persistent: bool) {
        let id = self.counter.get_untracked();
        self.counter.set(id + 1);
        let toast = Toast {
            id,
            message: message.into(),
            level,
            persistent,
        };
        self.toasts.update(|list| list.push(toast));

        if !persistent {
            let toasts = self.toasts;
            set_timeout(
                move || {
                    toasts.update(|list| list.retain(|t| t.id != id));
                },
                std::time::Duration::from_secs(4),
            );
        }
    }

    pub fn dismiss(&self, id: u64) {
        self.toasts.update(|list| list.retain(|t| t.id != id));
    }

    pub fn success(&self, msg: impl Into<String>) {
        self.push(msg, ToastLevel::Success);
    }

    pub fn error(&self, msg: impl Into<String>) {
        self.push(msg, ToastLevel::Error);
    }

    pub fn warning(&self, msg: impl Into<String>) {
        self.push(msg, ToastLevel::Warning);
    }

    pub fn info(&self, msg: impl Into<String>) {
        self.push(msg, ToastLevel::Info);
    }
}

#[component]
pub fn ToastContainer() -> impl IntoView {
    let toaster = use_context::<Toaster>().expect("Toaster not provided");

    view! {
        <div
            role="alert"
            aria-live="polite"
            class="fixed top-4 right-4 z-[100] space-y-2 pointer-events-none"
        >
            <For each=move || toaster.toasts.get() key=|t| t.id let:toast>
                <ToastItem toast=toast toaster=toaster />
            </For>
        </div>
    }
}

#[component]
fn ToastItem(toast: Toast, toaster: Toaster) -> impl IntoView {
    let (icon, bg, border, text) = match toast.level {
        ToastLevel::Success => (
            "\u{2713}",
            "bg-success-dim",
            "border-primary/20",
            "text-primary",
        ),
        ToastLevel::Error => (
            "\u{2717}",
            "bg-destructive-dim",
            "border-destructive/20",
            "text-destructive",
        ),
        ToastLevel::Warning => ("!", "bg-warning-dim", "border-warning/20", "text-warning"),
        ToastLevel::Info => ("\u{2139}", "bg-info-dim", "border-info/20", "text-info"),
    };

    let id = toast.id;

    view! {
        <div class=format!(
            "pointer-events-auto flex items-center gap-3 px-4 py-3 rounded-lg border backdrop-blur-sm \
             animate-in fade-in-0 slide-in-from-right duration-300 {} {} {}",
            bg,
            border,
            text,
        )>
            <span class="text-sm font-bold">{icon}</span>
            <span class="text-sm font-medium flex-1">{toast.message}</span>
            <button
                class="text-current opacity-60 hover:opacity-100 transition-opacity text-sm font-bold ml-2"
                on:click=move |_| toaster.dismiss(id)
                aria-label="Dismiss"
            >
                "\u{00d7}"
            </button>
        </div>
    }
}
