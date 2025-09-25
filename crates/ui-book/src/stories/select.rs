// @component Select
use holt_book::{story, variant};
use holt_ui::visual::{
    Select, SelectContent, SelectItem, SelectLabel, SelectSeparator, SelectTrigger, SelectValue,
};
use leptos::prelude::*;

#[variant]
fn default() -> AnyView {
    let value = RwSignal::new(None);

    view! {
        <div class="w-80 space-y-2">
            <Select value=value>
                <SelectTrigger>
                    <SelectValue placeholder="Select a fruit" />
                </SelectTrigger>
                <SelectContent>
                    <SelectItem value="apple".to_string()>"Apple"</SelectItem>
                    <SelectItem value="banana".to_string()>"Banana"</SelectItem>
                    <SelectItem value="cherry".to_string()>"Cherry"</SelectItem>
                    <SelectItem value="date".to_string()>"Date"</SelectItem>
                    <SelectItem value="elderberry".to_string()>"Elderberry"</SelectItem>
                </SelectContent>
            </Select>
            <p class="text-sm text-gray-600">
                "Selected: " {move || value.get().unwrap_or_else(|| "None".to_string())}
            </p>
        </div>
    }
    .into_any()
}

#[variant]
fn with_groups() -> AnyView {
    let value = RwSignal::new(None);

    view! {
        <div class="w-80 space-y-2">
            <Select value=value>
                <SelectTrigger>
                    <SelectValue placeholder="Select a framework" />
                </SelectTrigger>
                <SelectContent>
                    <SelectLabel>"Frontend"</SelectLabel>
                    <SelectItem value="react".to_string()>"React"</SelectItem>
                    <SelectItem value="vue".to_string()>"Vue"</SelectItem>
                    <SelectItem value="svelte".to_string()>"Svelte"</SelectItem>
                    <SelectItem value="solid".to_string()>"Solid"</SelectItem>

                    <SelectSeparator />

                    <SelectLabel>"Backend"</SelectLabel>
                    <SelectItem value="node".to_string()>"Node.js"</SelectItem>
                    <SelectItem value="python".to_string()>"Python"</SelectItem>
                    <SelectItem value="rust".to_string()>"Rust"</SelectItem>
                    <SelectItem value="go".to_string()>"Go"</SelectItem>

                    <SelectSeparator />

                    <SelectLabel>"Database"</SelectLabel>
                    <SelectItem value="postgres".to_string()>"PostgreSQL"</SelectItem>
                    <SelectItem value="mysql".to_string()>"MySQL"</SelectItem>
                    <SelectItem value="sqlite".to_string()>"SQLite"</SelectItem>
                </SelectContent>
            </Select>
            <p class="text-sm text-gray-600">
                "Selected: " {move || value.get().unwrap_or_else(|| "None".to_string())}
            </p>
        </div>
    }
    .into_any()
}

#[variant]
fn disabled() -> AnyView {
    let value = RwSignal::new(Some("apple".to_string()));

    view! {
        <div class="w-80">
            <Select value=value disabled=true>
                <SelectTrigger>
                    <SelectValue placeholder="Select a fruit" />
                </SelectTrigger>
                <SelectContent>
                    <SelectItem value="apple".to_string()>"Apple"</SelectItem>
                    <SelectItem value="banana".to_string()>"Banana"</SelectItem>
                    <SelectItem value="cherry".to_string()>"Cherry"</SelectItem>
                </SelectContent>
            </Select>
        </div>
    }
    .into_any()
}

#[variant]
fn with_disabled_items() -> AnyView {
    let value = RwSignal::new(None);

    view! {
        <div class="w-80">
            <Select value=value>
                <SelectTrigger>
                    <SelectValue placeholder="Select an option" />
                </SelectTrigger>
                <SelectContent>
                    <SelectItem value="available1".to_string()>"Available Option 1"</SelectItem>
                    <SelectItem value="available2".to_string()>"Available Option 2"</SelectItem>
                    <SelectItem value="disabled1".to_string() disabled=true>
                        "Disabled Option 1"
                    </SelectItem>
                    <SelectItem value="available3".to_string()>"Available Option 3"</SelectItem>
                    <SelectItem value="disabled2".to_string() disabled=true>
                        "Disabled Option 2"
                    </SelectItem>
                </SelectContent>
            </Select>
        </div>
    }
    .into_any()
}

#[variant]
fn countries() -> AnyView {
    let value = RwSignal::new(None);

    view! {
        <div class="w-80 space-y-2">
            <Select value=value>
                <SelectTrigger>
                    <SelectValue placeholder="Select a country" />
                </SelectTrigger>
                <SelectContent>
                    <SelectItem value="us".to_string()>"🇺🇸 United States"</SelectItem>
                    <SelectItem value="uk".to_string()>"🇬🇧 United Kingdom"</SelectItem>
                    <SelectItem value="ca".to_string()>"🇨🇦 Canada"</SelectItem>
                    <SelectItem value="au".to_string()>"🇦🇺 Australia"</SelectItem>
                    <SelectItem value="de".to_string()>"🇩🇪 Germany"</SelectItem>
                    <SelectItem value="fr".to_string()>"🇫🇷 France"</SelectItem>
                    <SelectItem value="jp".to_string()>"🇯🇵 Japan"</SelectItem>
                    <SelectItem value="kr".to_string()>"🇰🇷 South Korea"</SelectItem>
                    <SelectItem value="in".to_string()>"🇮🇳 India"</SelectItem>
                    <SelectItem value="br".to_string()>"🇧🇷 Brazil"</SelectItem>
                </SelectContent>
            </Select>
            <p class="text-sm text-gray-600">
                "Selected: " {move || value.get().unwrap_or_else(|| "None".to_string())}
            </p>
        </div>
    }
    .into_any()
}

#[variant]
fn form_example() -> AnyView {
    let framework = RwSignal::new(None);
    let language = RwSignal::new(None);
    let experience = RwSignal::new(None);

    view! {
        <div class="w-80 space-y-4">
            <div class="space-y-2">
                <label class="text-sm font-medium">"Preferred Framework"</label>
                <Select value=framework>
                    <SelectTrigger>
                        <SelectValue placeholder="Choose framework" />
                    </SelectTrigger>
                    <SelectContent>
                        <SelectItem value="react".to_string()>"React"</SelectItem>
                        <SelectItem value="vue".to_string()>"Vue"</SelectItem>
                        <SelectItem value="angular".to_string()>"Angular"</SelectItem>
                        <SelectItem value="svelte".to_string()>"Svelte"</SelectItem>
                        <SelectItem value="leptos".to_string()>"Leptos"</SelectItem>
                    </SelectContent>
                </Select>
            </div>

            <div class="space-y-2">
                <label class="text-sm font-medium">"Programming Language"</label>
                <Select value=language>
                    <SelectTrigger>
                        <SelectValue placeholder="Choose language" />
                    </SelectTrigger>
                    <SelectContent>
                        <SelectItem value="javascript".to_string()>"JavaScript"</SelectItem>
                        <SelectItem value="typescript".to_string()>"TypeScript"</SelectItem>
                        <SelectItem value="rust".to_string()>"Rust"</SelectItem>
                        <SelectItem value="python".to_string()>"Python"</SelectItem>
                        <SelectItem value="go".to_string()>"Go"</SelectItem>
                    </SelectContent>
                </Select>
            </div>

            <div class="space-y-2">
                <label class="text-sm font-medium">"Experience Level"</label>
                <Select value=experience>
                    <SelectTrigger>
                        <SelectValue placeholder="Choose experience" />
                    </SelectTrigger>
                    <SelectContent>
                        <SelectItem value="beginner".to_string()>"Beginner (0-1 years)"</SelectItem>
                        <SelectItem value="intermediate"
                            .to_string()>"Intermediate (2-4 years)"</SelectItem>
                        <SelectItem value="advanced".to_string()>"Advanced (5-7 years)"</SelectItem>
                        <SelectItem value="expert".to_string()>"Expert (8+ years)"</SelectItem>
                    </SelectContent>
                </Select>
            </div>

            <div class="pt-2 text-sm text-gray-600">
                <p>"Framework: " {move || framework.get().unwrap_or_else(|| "None".to_string())}</p>
                <p>"Language: " {move || language.get().unwrap_or_else(|| "None".to_string())}</p>
                <p>
                    "Experience: " {move || experience.get().unwrap_or_else(|| "None".to_string())}
                </p>
            </div>
        </div>
    }
    .into_any()
}

include!(concat!(env!("OUT_DIR"), "/stories/select_source.rs"));

#[story(id = "select", name = "Select", extra_docs = SELECT_SOURCE)]
/// Select components allow users to choose from a list of options
const SELECT_STORY: () = &[
    default,
    with_groups,
    disabled,
    with_disabled_items,
    countries,
    form_example,
];
