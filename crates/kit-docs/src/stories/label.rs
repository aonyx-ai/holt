// @component Label
use holt_book::{story, variant};
use holt_kit::visual::{Checkbox, Input, Label};
use leptos::prelude::*;

#[variant]
fn default() -> AnyView {
    view! {
        <div class="space-y-2">
            <Label r#for="email">"Email"</Label>
            <Input id="email" name="email" placeholder="Enter your email" />
        </div>
    }
    .into_any()
}

#[variant]
fn with_checkbox() -> AnyView {
    let checked = RwSignal::new(false);
    view! {
        <div class="flex items-center space-x-2">
            <Checkbox id="terms" checked=checked />
            <Label r#for="terms">"Accept terms and conditions"</Label>
        </div>
    }
    .into_any()
}

#[variant]
fn required_field() -> AnyView {
    view! {
        <div class="space-y-2">
            <Label r#for="name">"Full Name" <span class="text-destructive">"*"</span></Label>
            <Input id="name" name="name" placeholder="John Doe" required=true />
        </div>
    }
    .into_any()
}

#[variant]
fn disabled_field() -> AnyView {
    let value = RwSignal::new("Cannot edit".to_string());

    view! {
        <div class="space-y-2">
            <Label r#for="disabled-input">"Disabled Field"</Label>
            <Input id="disabled-input" value=value disabled=true />
        </div>
    }
    .into_any()
}

#[variant]
fn multiple_labels() -> AnyView {
    let newsletter = RwSignal::new(true);
    let marketing = RwSignal::new(false);

    view! {
        <div class="space-y-4">
            <div class="space-y-2">
                <Label r#for="username">"Username"</Label>
                <Input id="username" placeholder="johndoe" />
            </div>

            <div class="space-y-2">
                <Label r#for="password">"Password"</Label>
                <Input id="password" r#type="password" placeholder="••••••••" />
            </div>

            <div class="space-y-3">
                <div class="flex items-center space-x-2">
                    <Checkbox id="newsletter" checked=newsletter />
                    <Label r#for="newsletter">"Subscribe to newsletter"</Label>
                </div>

                <div class="flex items-center space-x-2">
                    <Checkbox id="marketing" checked=marketing />
                    <Label r#for="marketing">"Receive marketing emails"</Label>
                </div>
            </div>
        </div>
    }
    .into_any()
}

#[variant]
fn form_layout() -> AnyView {
    view! {
        <div class="space-y-6 max-w-sm">
            <div class="space-y-2">
                <Label r#for="first-name">"First Name"</Label>
                <Input id="first-name" placeholder="First name" />
            </div>

            <div class="space-y-2">
                <Label r#for="last-name">"Last Name"</Label>
                <Input id="last-name" placeholder="Last name" />
            </div>

            <div class="space-y-2">
                <Label r#for="email-address">"Email Address"</Label>
                <Input id="email-address" r#type="email" placeholder="you@example.com" />
            </div>

            <div class="space-y-2">
                <Label r#for="phone">"Phone Number"</Label>
                <Input id="phone" r#type="tel" placeholder="+1 (555) 000-0000" />
            </div>
        </div>
    }
    .into_any()
}

include!(concat!(env!("OUT_DIR"), "/stories/label_source.rs"));

#[story(id = "label", name = "Label", extra_docs = LABEL_SOURCE)]
/// Labels provide accessible names for form controls
const LABEL_STORY: () = &[
    default,
    with_checkbox,
    required_field,
    disabled_field,
    multiple_labels,
    form_layout,
];
