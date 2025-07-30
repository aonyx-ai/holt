use holt_book::{story, variant};
use holt_ui::visual::{
    Button, ButtonVariant, Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle,
};
use leptos::prelude::*;
use leptos_icons::Icon;

#[variant]
fn simple() -> AnyView {
    view! {
        <Card>
            <CardHeader>
                <CardTitle>Card Title</CardTitle>
                <CardDescription>Card Description</CardDescription>
            </CardHeader>
            <CardContent>
                <p>Card content goes here.</p>
            </CardContent>
        </Card>
    }
    .into_any()
}

#[variant]
fn with_footer() -> AnyView {
    view! {
        <Card>
            <CardHeader>
                <CardTitle>Project Setup</CardTitle>
                <CardDescription>Get started with your new project in minutes.</CardDescription>
            </CardHeader>
            <CardContent>
                <p class="text-sm text-muted-foreground">
                    Follow these steps to configure your project and start building amazing things.
                </p>
            </CardContent>
            <CardFooter>
                <Button variant=ButtonVariant::Outline class="mr-2">Cancel</Button>
                <Button>Get Started</Button>
            </CardFooter>
        </Card>
    }
    .into_any()
}

#[variant]
fn notification_card() -> AnyView {
    view! {
        <Card class="w-96">
            <CardHeader class="pb-3">
                <div class="flex items-center space-x-4">
                    <Icon icon=icondata::LuBell attr:class="h-8 w-8" />
                    <div>
                        <CardTitle class="text-base">New Notification</CardTitle>
                        <CardDescription>You have a new message</CardDescription>
                    </div>
                </div>
            </CardHeader>
            <CardContent>
                <p class="text-sm">
                    Your deployment has completed successfully.
                    All systems are running normally.
                </p>
            </CardContent>
            <CardFooter class="pt-3">
                <Button variant=ButtonVariant::Ghost class="text-xs">Mark as read</Button>
            </CardFooter>
        </Card>
    }
    .into_any()
}

#[variant]
fn stats_card() -> AnyView {
    view! {
        <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
            <Card>
                <CardHeader class="flex flex-row items-center justify-between space-y-0 pb-2">
                    <CardTitle class="text-sm font-medium">Total Revenue</CardTitle>
                    <Icon icon=icondata::LuDollarSign attr:class="h-4 w-4 text-muted-foreground" />
                </CardHeader>
                <CardContent>
                    <div class="text-2xl font-bold">$45,231.89</div>
                    <p class="text-xs text-muted-foreground">
                        +20.1% from last month
                    </p>
                </CardContent>
            </Card>
            <Card>
                <CardHeader class="flex flex-row items-center justify-between space-y-0 pb-2">
                    <CardTitle class="text-sm font-medium">Subscriptions</CardTitle>
                    <Icon icon=icondata::LuUsers attr:class="h-4 w-4 text-muted-foreground" />
                </CardHeader>
                <CardContent>
                    <div class="text-2xl font-bold">+2,350</div>
                    <p class="text-xs text-muted-foreground">
                        +180.1% from last month
                    </p>
                </CardContent>
            </Card>
            <Card>
                <CardHeader class="flex flex-row items-center justify-between space-y-0 pb-2">
                    <CardTitle class="text-sm font-medium">Active Now</CardTitle>
                    <Icon icon=icondata::LuActivity attr:class="h-4 w-4 text-muted-foreground" />
                </CardHeader>
                <CardContent>
                    <div class="text-2xl font-bold">+573</div>
                    <p class="text-xs text-muted-foreground">
                        +201 since last hour
                    </p>
                </CardContent>
            </Card>
        </div>
    }
    .into_any()
}

#[variant]
fn profile_card() -> AnyView {
    view! {
        <Card class="w-80">
            <CardHeader class="text-center">
                <div class="mx-auto mb-4 h-20 w-20 rounded-full bg-gradient-to-r from-blue-500 to-purple-600 flex items-center justify-center">
                    <Icon icon=icondata::LuUser attr:class="h-10 w-10 text-white" />
                </div>
                <CardTitle>John Doe</CardTitle>
                <CardDescription>Software Engineer</CardDescription>
            </CardHeader>
            <CardContent class="text-center">
                <p class="text-sm text-muted-foreground mb-4">
                    Passionate about building great user experiences and scalable systems.
                </p>
                <div class="flex justify-center space-x-4 text-sm">
                    <div>
                        <div class="font-semibold">1.2k</div>
                        <div class="text-muted-foreground">Followers</div>
                    </div>
                    <div>
                        <div class="font-semibold">847</div>
                        <div class="text-muted-foreground">Following</div>
                    </div>
                </div>
            </CardContent>
            <CardFooter>
                <Button class="w-full">Follow</Button>
            </CardFooter>
        </Card>
    }
    .into_any()
}

#[variant]
fn feature_highlight() -> AnyView {
    view! {
        <Card class="relative overflow-hidden">
            <div class="absolute top-0 right-0 w-20 h-20 bg-gradient-to-br from-yellow-400 to-orange-500 rounded-bl-full" />
            <CardHeader>
                <div class="flex items-start justify-between">
                    <div>
                        <CardTitle class="flex items-center space-x-2">
                            <Icon icon=icondata::LuStar attr:class="h-5 w-5 text-yellow-500" />
                            <span>Premium Feature</span>
                        </CardTitle>
                        <CardDescription>Unlock advanced capabilities</CardDescription>
                    </div>
                </div>
            </CardHeader>
            <CardContent>
                <ul class="space-y-2 text-sm">
                    <li class="flex items-center space-x-2">
                        <Icon icon=icondata::LuCheck attr:class="h-4 w-4 text-green-500" />
                        <span>Advanced analytics</span>
                    </li>
                    <li class="flex items-center space-x-2">
                        <Icon icon=icondata::LuCheck attr:class="h-4 w-4 text-green-500" />
                        <span>Priority support</span>
                    </li>
                    <li class="flex items-center space-x-2">
                        <Icon icon=icondata::LuCheck attr:class="h-4 w-4 text-green-500" />
                        <span>Custom integrations</span>
                    </li>
                </ul>
            </CardContent>
            <CardFooter>
                <Button class="w-full">Upgrade Now</Button>
            </CardFooter>
        </Card>
    }
    .into_any()
}

#[variant]
fn minimal() -> AnyView {
    view! {
        <Card class="w-64">
            <CardContent class="pt-6">
                <div class="text-center">
                    <Icon icon=icondata::LuPackage attr:class="mx-auto h-12 w-12 text-muted-foreground mb-4" />
                    <h3 class="font-semibold mb-2">Empty State</h3>
                    <p class="text-sm text-muted-foreground">
                        No items to display yet.
                    </p>
                </div>
            </CardContent>
        </Card>
    }
    .into_any()
}

#[variant]
fn interactive() -> AnyView {
    let count = RwSignal::new(0);

    view! {
        <Card
            class="w-72 cursor-pointer transition-all hover:shadow-lg"
            on:click=move |_| count.update(|n| *n += 1)
        >
            <CardHeader>
                <CardTitle class="flex items-center justify-between">
                    <span>Interactive Card</span>
                    <Icon icon=icondata::LuMousePointer attr:class="h-4 w-4" />
                </CardTitle>
                <CardDescription>Click the button to see changes</CardDescription>
            </CardHeader>
            <CardContent>
                <div class="text-center">
                    <div class="text-3xl font-bold mb-2">{move || count.get()}</div>
                    <p class="text-sm text-muted-foreground">Button clicks</p>
                </div>
            </CardContent>
            <CardFooter>
                <Button class="w-full">
                    Click me!
                </Button>
            </CardFooter>
        </Card>
    }
    .into_any()
}

include!(concat!(env!("OUT_DIR"), "/stories/card_source.rs"));

#[story(id = "card", name = "Card", extra_docs = CARD_SOURCE)]
/// Cards are flexible containers for grouping related content and actions
const CARD_STORY: () = &[
    simple,
    with_footer,
    notification_card,
    stats_card,
    profile_card,
    feature_highlight,
    minimal,
    interactive,
];
