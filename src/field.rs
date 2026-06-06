use leptos::prelude::*;

/// Layout orientation for a [`Field`] — shadcn's `data-orientation` mapped to a
/// Rust enum (CONVENTIONS rule 4). `Responsive` stacks vertically until the
/// enclosing [`FieldGroup`] container hits the `@md` breakpoint, then goes
/// horizontal.
#[derive(Default, Clone, Copy, PartialEq)]
pub enum FieldOrientation {
    #[default]
    Vertical,
    Horizontal,
    Responsive,
}

impl FieldOrientation {
    fn classes(self) -> &'static str {
        match self {
            FieldOrientation::Vertical => "flex-col [&>*]:w-full [&>.sr-only]:w-auto",
            FieldOrientation::Horizontal => "flex-row items-center",
            FieldOrientation::Responsive => {
                "flex-col @md/field-group:flex-row @md/field-group:items-center [&>*]:w-full @md/field-group:[&>*]:w-auto [&>.sr-only]:w-auto"
            }
        }
    }
}

/// Text style for a [`FieldLegend`] — shadcn's `data-[variant=legend|label]`.
#[derive(Default, Clone, Copy, PartialEq)]
pub enum FieldLegendVariant {
    #[default]
    Legend,
    Label,
}

impl FieldLegendVariant {
    fn classes(self) -> &'static str {
        match self {
            FieldLegendVariant::Legend => "text-base",
            FieldLegendVariant::Label => "text-sm",
        }
    }
}

#[component]
pub fn FieldSet(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    view! { <fieldset class=format!("flex flex-col gap-6 {}", class)>{children()}</fieldset> }
}

#[component]
pub fn FieldLegend(
    #[prop(optional)] variant: FieldLegendVariant,
    #[prop(optional, into)] class: String,
    children: Children,
) -> impl IntoView {
    view! {
        <legend class=format!(
            "mb-3 font-medium {} {}",
            variant.classes(),
            class,
        )>{children()}</legend>
    }
}

#[component]
pub fn FieldGroup(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    view! {
        <div class=format!(
            "group/field-group @container/field-group flex w-full flex-col gap-7 [&>[data-slot=field-group]]:gap-4 {}",
            class,
        )>{children()}</div>
    }
}

#[component]
pub fn Field(
    #[prop(optional)] orientation: FieldOrientation,
    #[prop(optional, into)] class: String,
    children: Children,
) -> impl IntoView {
    view! {
        <div
            role="group"
            class=format!(
                "group/field flex w-full gap-3 data-[invalid=true]:text-destructive {} {}",
                orientation.classes(),
                class,
            )
        >
            {children()}
        </div>
    }
}

#[component]
pub fn FieldContent(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    view! {
        <div class=format!(
            "group/field-content flex flex-1 flex-col gap-1.5 leading-snug {}",
            class,
        )>{children()}</div>
    }
}

#[component]
pub fn FieldLabel(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    view! {
        <label class=format!(
            "group/field-label peer/field-label flex w-fit gap-2 leading-snug font-medium text-sm group-data-[disabled=true]/field:opacity-50 {}",
            class,
        )>{children()}</label>
    }
}

#[component]
pub fn FieldTitle(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    view! {
        <div class=format!(
            "flex w-fit items-center gap-2 text-sm leading-snug font-medium group-data-[disabled=true]/field:opacity-50 {}",
            class,
        )>{children()}</div>
    }
}

#[component]
pub fn FieldDescription(
    #[prop(optional, into)] class: String,
    children: Children,
) -> impl IntoView {
    view! {
        <p class=format!(
            "text-sm leading-normal font-normal text-muted-foreground last:mt-0 nth-last-2:-mt-1 [&>a]:underline [&>a]:underline-offset-4 [&>a:hover]:text-primary {}",
            class,
        )>{children()}</p>
    }
}

#[component]
pub fn FieldSeparator(
    #[prop(optional, into)] class: String,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    view! {
        <div class=format!("relative -my-2 h-5 text-sm {}", class)>
            <div
                role="separator"
                class="shrink-0 bg-border h-[1px] w-full absolute inset-0 top-1/2"
            ></div>
            {children
                .map(|children| {
                    view! {
                        <span class="relative mx-auto block w-fit bg-background px-2 text-muted-foreground">
                            {children()}
                        </span>
                    }
                })}
        </div>
    }
}

#[component]
pub fn FieldError(
    #[prop(optional, into)] message: String,
    #[prop(optional, into)] errors: Vec<String>,
    #[prop(optional, into)] class: String,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    // Precedence mirrors shadcn: explicit children win, then a single `message`,
    // then a de-duplicated bullet list from `errors`. Render nothing when empty.
    let mut unique: Vec<String> = Vec::new();
    for e in errors.into_iter().filter(|e| !e.is_empty()) {
        if !unique.contains(&e) {
            unique.push(e);
        }
    }

    let body = if let Some(children) = children {
        children().into_any()
    } else if !message.is_empty() {
        message.into_any()
    } else if unique.len() == 1 {
        unique.into_iter().next().unwrap().into_any()
    } else if !unique.is_empty() {
        view! {
            <ul class="ml-4 flex list-disc flex-col gap-1">
                {unique.into_iter().map(|e| view! { <li>{e}</li> }).collect_view()}
            </ul>
        }
        .into_any()
    } else {
        return ().into_any();
    };

    view! {
        <div role="alert" class=format!("text-sm font-normal text-destructive {}", class)>
            {body}
        </div>
    }
    .into_any()
}
