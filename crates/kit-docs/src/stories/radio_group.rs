// @component radio_group
#[cfg(feature = "e2e")]
mod e2e;

use holt_book::{story, variant};
use holt_kit::visual::{Label, RadioGroup, RadioGroupItem};
use leptos::prelude::*;

#[variant]
fn default() -> AnyView {
    let value = RwSignal::new(None::<String>);
    view! {
        <RadioGroup value=value>
            <div class="flex items-center space-x-2">
                <RadioGroupItem value="option1" />
                <Label r#for="option1">"Option One"</Label>
            </div>
            <div class="flex items-center space-x-2">
                <RadioGroupItem value="option2" />
                <Label r#for="option2">"Option Two"</Label>
            </div>
            <div class="flex items-center space-x-2">
                <RadioGroupItem value="option3" />
                <Label r#for="option3">"Option Three"</Label>
            </div>
        </RadioGroup>
    }
    .into_any()
}

#[variant]
fn with_default_value() -> AnyView {
    let value = RwSignal::new(Some("comfortable".to_string()));
    view! {
        <RadioGroup value=value>
            <div class="flex items-center space-x-2">
                <RadioGroupItem value="default" />
                <Label r#for="default">"Default"</Label>
            </div>
            <div class="flex items-center space-x-2">
                <RadioGroupItem value="comfortable" />
                <Label r#for="comfortable">"Comfortable"</Label>
            </div>
            <div class="flex items-center space-x-2">
                <RadioGroupItem value="compact" />
                <Label r#for="compact">"Compact"</Label>
            </div>
        </RadioGroup>
    }
    .into_any()
}

#[variant]
fn disabled() -> AnyView {
    let value = RwSignal::new(Some("option1".to_string()));
    view! {
        <RadioGroup value=value disabled=true>
            <div class="flex items-center space-x-2">
                <RadioGroupItem value="option1" />
                <Label r#for="option1">"Option One"</Label>
            </div>
            <div class="flex items-center space-x-2">
                <RadioGroupItem value="option2" />
                <Label r#for="option2">"Option Two"</Label>
            </div>
        </RadioGroup>
    }
    .into_any()
}

#[variant]
fn interactive() -> AnyView {
    let value = RwSignal::new(None::<String>);
    view! {
        <div class="flex flex-col space-y-4">
            <RadioGroup value=value>
                <div class="flex items-center space-x-2">
                    <RadioGroupItem value="small" />
                    <Label r#for="small">"Small"</Label>
                </div>
                <div class="flex items-center space-x-2">
                    <RadioGroupItem value="medium" />
                    <Label r#for="medium">"Medium"</Label>
                </div>
                <div class="flex items-center space-x-2">
                    <RadioGroupItem value="large" />
                    <Label r#for="large">"Large"</Label>
                </div>
            </RadioGroup>
            <p class="text-sm text-muted-foreground">
                "Selected: " {move || value.get().unwrap_or_else(|| "none".to_string())}
            </p>
        </div>
    }
    .into_any()
}

include!(concat!(env!("OUT_DIR"), "/stories/radio_group_source.rs"));

#[story(id = "radio-group", name = "Radio Group", extra_docs = RADIO_GROUP_SOURCE)]
/// Radio groups allow users to select one option from a set of choices
const RADIO_GROUP_STORY: () = &[default, with_default_value, disabled, interactive];
