use leptos::html;
use leptos::prelude::*;
use leptos_icons::Icon;
use tailwind_fuse::*;

#[derive(TwClass)]
#[tw(
    class = "peer h-4 w-4 shrink-0 rounded-sm border border-input ring-offset-background focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50 data-[state=checked]:bg-primary data-[state=checked]:text-primary-foreground data-[state=checked]:border-primary"
)]
struct CheckboxStyle {
    size: CheckboxSize,
}

#[derive(TwVariant)]
pub enum CheckboxSize {
    #[tw(default, class = "h-4 w-4")]
    Default,
    #[tw(class = "h-3 w-3")]
    Sm,
    #[tw(class = "h-5 w-5")]
    Lg,
}

#[component]
pub fn Checkbox(
    #[prop(optional)] class: &'static str,
    #[prop(optional)] size: CheckboxSize,
    #[prop(optional)] checked: RwSignal<bool>,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] id: Option<&'static str>,
    #[prop(optional)] name: Option<&'static str>,
) -> impl IntoView {
    let final_class = CheckboxStyle { size }.with_class(class);
    let element: NodeRef<html::Input> = NodeRef::new();

    let checkbox_class = move || {
        tw_merge!(
            final_class.clone(),
            checked
                .get()
                .then_some("bg-primary text-primary-foreground border-primary"),
        )
    };

    view! {
        <div class="relative inline-flex items-center">
            <input
                type="checkbox"
                node_ref=element
                class="sr-only"
                bind:checked=checked
                disabled=disabled
                id=id
                name=name
            />
            <div
                class=checkbox_class
                class:cursor-pointer=move || !disabled
                on:click=move |_| {
                    if !disabled {
                        if let Some(el) = element.get() { el.click() }
                    }
                }
            >
                <Show when=move || checked.get()>
                    <div class="flex items-center justify-center text-current">
                        <Icon
                            icon=icondata::LuCheck
                            attr:class=match size {
                                CheckboxSize::Sm => "h-2.5 w-2.5",
                                CheckboxSize::Default => "h-3 w-3",
                                CheckboxSize::Lg => "h-4 w-4",
                            }
                        />
                    </div>
                </Show>
            </div>
        </div>
    }
}
