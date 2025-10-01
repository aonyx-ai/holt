use leptos::prelude::*;
use tailwind_fuse::*;

#[derive(TwClass)]
#[tw(
    class = "flex min-h-[80px] w-full rounded-md border border-input bg-background px-3 text-sm ring-offset-background placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-[3px] focus-visible:ring-ring/50 disabled:cursor-not-allowed disabled:opacity-50"
)]
struct TextareaStyle {
    size: TextareaSize,
}

#[derive(TwVariant)]
pub enum TextareaSize {
    #[tw(default, class = "py-2 min-h-[80px]")]
    Default,
    #[tw(class = "py-1.5 text-sm min-h-[60px]")]
    Sm,
    #[tw(class = "py-2.5 text-base min-h-[100px]")]
    Lg,
}

#[component]
pub fn Textarea(
    #[prop(optional)] class: &'static str,
    #[prop(optional)] size: TextareaSize,
    /// Two‑way bound via `bind:value`.
    #[prop(optional, default = RwSignal::new(String::new()))]
    value: RwSignal<String>,
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(optional, into)] readonly: Signal<bool>,
    #[prop(optional, into)] required: Signal<bool>,
    #[prop(optional_no_strip, into)] id: Option<&'static str>,
    #[prop(optional_no_strip, into)] name: Option<&'static str>,
    #[prop(optional_no_strip, into)] placeholder: Option<&'static str>,
    #[prop(optional_no_strip, into)] rows: Option<u32>,
    #[prop(optional_no_strip, into)] cols: Option<u32>,
) -> impl IntoView {
    let class = TextareaStyle { size }.with_class(class);

    view! {
        <textarea
            class=class
            bind:value=value
            id=id
            name=name
            placeholder=placeholder
            rows=rows
            cols=cols
            disabled=disabled
            readonly=readonly
            required=required
            data-invalid=move || {
                if required.get() && value.get().is_empty() { Some("true") } else { None }
            }
        />
    }
}
