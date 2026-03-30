// @component Tabs
#[cfg(feature = "e2e")]
mod e2e;

use holt_book::{story, variant};
use holt_kit::visual::{Tabs, TabsContent, TabsList, TabsTrigger};
use leptos::prelude::*;

#[variant]
fn basic() -> AnyView {
    view! {
        <Tabs default_value="account" class="w-full max-w-md">
            <TabsList>
                <TabsTrigger value="account">"Account"</TabsTrigger>
                <TabsTrigger value="password">"Password"</TabsTrigger>
            </TabsList>
            <TabsContent value="account">
                <div class="rounded-md border p-4 text-sm">
                    <p class="font-medium">"Account"</p>
                    <p class="text-muted-foreground">"Make changes to your account settings."</p>
                </div>
            </TabsContent>
            <TabsContent value="password">
                <div class="rounded-md border p-4 text-sm">
                    <p class="font-medium">"Password"</p>
                    <p class="text-muted-foreground">"Change your password here."</p>
                </div>
            </TabsContent>
        </Tabs>
    }
    .into_any()
}

#[variant]
fn three_tabs() -> AnyView {
    view! {
        <Tabs default_value="overview" class="w-full max-w-md">
            <TabsList>
                <TabsTrigger value="overview">"Overview"</TabsTrigger>
                <TabsTrigger value="analytics">"Analytics"</TabsTrigger>
                <TabsTrigger value="reports">"Reports"</TabsTrigger>
            </TabsList>
            <TabsContent value="overview">
                <div class="rounded-md border p-4 text-sm text-muted-foreground">
                    "Your project overview will appear here."
                </div>
            </TabsContent>
            <TabsContent value="analytics">
                <div class="rounded-md border p-4 text-sm text-muted-foreground">
                    "Analytics data will appear here."
                </div>
            </TabsContent>
            <TabsContent value="reports">
                <div class="rounded-md border p-4 text-sm text-muted-foreground">
                    "Generated reports will appear here."
                </div>
            </TabsContent>
        </Tabs>
    }
    .into_any()
}

#[variant]
fn with_disabled() -> AnyView {
    view! {
        <Tabs default_value="active" class="w-full max-w-md">
            <TabsList>
                <TabsTrigger value="active">"Active"</TabsTrigger>
                <TabsTrigger value="disabled" disabled=true>
                    "Disabled"
                </TabsTrigger>
                <TabsTrigger value="other">"Other"</TabsTrigger>
            </TabsList>
            <TabsContent value="active">
                <div class="rounded-md border p-4 text-sm text-muted-foreground">
                    "This tab is active."
                </div>
            </TabsContent>
            <TabsContent value="other">
                <div class="rounded-md border p-4 text-sm text-muted-foreground">
                    "The disabled tab cannot be selected."
                </div>
            </TabsContent>
        </Tabs>
    }
    .into_any()
}

include!(concat!(env!("OUT_DIR"), "/stories/tabs_source.rs"));

#[story(id = "tabs", name = "Tabs", extra_docs = TABS_SOURCE)]
/// A set of layered content sections displayed one at a time
const TABS_STORY: () = &[basic, three_tabs, with_disabled];
