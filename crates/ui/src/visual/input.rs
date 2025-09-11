use leptos::prelude::*;
use tailwind_fuse::*;

#[derive(TwClass)]
#[tw(
    class = "flex w-full rounded-md border border-input bg-background px-3 text-sm ring-offset-background placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-[3px] focus-visible:ring-ring/50 disabled:cursor-not-allowed disabled:opacity-50 file:border-0 file:bg-transparent file:text-sm file:font-medium"
)]
struct InputStyle {
    size: InputSize,
}

#[derive(TwVariant)]
pub enum InputSize {
    #[tw(default, class = "h-10 py-2")]
    Default,
    #[tw(class = "h-9 py-1.5 text-sm")]
    Sm,
    #[tw(class = "h-11 py-2.5 text-base")]
    Lg,
}

#[component]
pub fn Input(
    #[prop(optional)] class: &'static str,
    #[prop(optional)] size: InputSize,
    /// Two‑way bound via `bind:value`.
    #[prop(optional, default = RwSignal::new(String::new()))]
    value: RwSignal<String>,
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(optional, into)] readonly: Signal<bool>,
    #[prop(optional, into)] required: Signal<bool>,
    #[prop(optional_no_strip, into)] id: Option<&'static str>,
    #[prop(optional_no_strip, into)] name: Option<&'static str>,
    #[prop(optional_no_strip, into)] placeholder: Option<&'static str>,
    #[prop(optional_no_strip, into)] r#type: Option<&'static str>,
) -> impl IntoView {
    let class = InputStyle { size }.with_class(class);
    let ty = r#type.unwrap_or("text");

    view! {
        <input
            class=class
            bind:value=value
            type=ty
            id=id
            name=name
            placeholder=placeholder
            disabled=disabled
            readonly=readonly
            required=required
            data-invalid=move || if required.get() && value.get().is_empty() { Some("true") } else { None }
        />
    }
}
