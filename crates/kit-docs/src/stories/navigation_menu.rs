// @component navigation_menu
#[cfg(feature = "e2e")]
mod e2e;

use holt_book::{story, variant};
use holt_kit::visual::{
    NavigationMenu, NavigationMenuContent, NavigationMenuItem, NavigationMenuLink,
    NavigationMenuList, NavigationMenuTrigger,
};
use leptos::prelude::*;

#[variant]
fn basic() -> AnyView {
    view! {
        <NavigationMenu>
            <NavigationMenuList>
                <NavigationMenuItem id="getting-started">
                    <NavigationMenuTrigger>"Getting Started"</NavigationMenuTrigger>
                    <NavigationMenuContent>
                        <div class="grid gap-3 w-[400px]">
                            <NavigationMenuLink href="/docs/introduction">
                                <div class="font-medium">"Introduction"</div>
                                <p class="text-sm text-muted-foreground">
                                    "An overview of the project and its goals."
                                </p>
                            </NavigationMenuLink>
                            <NavigationMenuLink href="/docs/installation">
                                <div class="font-medium">"Installation"</div>
                                <p class="text-sm text-muted-foreground">
                                    "How to install and set up your environment."
                                </p>
                            </NavigationMenuLink>
                        </div>
                    </NavigationMenuContent>
                </NavigationMenuItem>
                <NavigationMenuItem id="components">
                    <NavigationMenuTrigger>"Components"</NavigationMenuTrigger>
                    <NavigationMenuContent>
                        <div class="grid gap-3 w-[400px]">
                            <NavigationMenuLink href="/docs/components/button">
                                <div class="font-medium">"Button"</div>
                                <p class="text-sm text-muted-foreground">
                                    "Displays a button or a component that looks like a button."
                                </p>
                            </NavigationMenuLink>
                            <NavigationMenuLink href="/docs/components/card">
                                <div class="font-medium">"Card"</div>
                                <p class="text-sm text-muted-foreground">
                                    "Displays a card with header, content, and footer."
                                </p>
                            </NavigationMenuLink>
                        </div>
                    </NavigationMenuContent>
                </NavigationMenuItem>
            </NavigationMenuList>
        </NavigationMenu>
    }
    .into_any()
}

#[variant]
fn with_plain_links() -> AnyView {
    view! {
        <NavigationMenu>
            <NavigationMenuList>
                <NavigationMenuItem id="docs">
                    <NavigationMenuTrigger>"Documentation"</NavigationMenuTrigger>
                    <NavigationMenuContent>
                        <div class="grid gap-3 w-[300px]">
                            <NavigationMenuLink href="/docs">"Overview"</NavigationMenuLink>
                            <NavigationMenuLink href="/docs/guides">"Guides"</NavigationMenuLink>
                            <NavigationMenuLink href="/docs/api">
                                "API Reference"
                            </NavigationMenuLink>
                        </div>
                    </NavigationMenuContent>
                </NavigationMenuItem>
                <NavigationMenuItem id="about-link">
                    <NavigationMenuLink
                        href="/about"
                        class="inline-flex h-9 w-max items-center justify-center rounded-md bg-background px-4 py-2 text-sm font-medium transition-colors hover:bg-accent hover:text-accent-foreground"
                    >
                        "About"
                    </NavigationMenuLink>
                </NavigationMenuItem>
            </NavigationMenuList>
        </NavigationMenu>
    }
    .into_any()
}

include!(concat!(
    env!("OUT_DIR"),
    "/stories/navigation_menu_source.rs"
));

#[story(
    id = "navigation-menu",
    name = "Navigation Menu",
    extra_docs = NAVIGATION_MENU_SOURCE
)]
const NAVIGATION_MENU_STORY: () = &[basic, with_plain_links];
