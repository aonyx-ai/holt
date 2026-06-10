use leptos::prelude::*;

/// Context for the root navigation menu, tracking which item (by id) is currently open.
#[derive(Copy, Clone)]
pub struct NavigationMenuContext {
    pub active_item: RwSignal<Option<&'static str>>,
}

impl NavigationMenuContext {
    pub fn new() -> Self {
        Self {
            active_item: RwSignal::new(None),
        }
    }

    pub fn open(&self, id: &'static str) {
        self.active_item.set(Some(id));
    }

    pub fn close(&self) {
        self.active_item.set(None);
    }

    pub fn is_open(&self, id: &'static str) -> bool {
        self.active_item.get().is_some_and(|active| active == id)
    }

    pub fn toggle(&self, id: &'static str) {
        if self.is_open(id) {
            self.close();
        } else {
            self.open(id);
        }
    }
}

/// Access the navigation menu context.
pub fn use_navigation_menu() -> NavigationMenuContext {
    use_context::<NavigationMenuContext>()
        .expect("use_navigation_menu must be called within NavigationMenuRoot")
}

/// Context for an individual navigation menu item, carrying its identifier.
#[derive(Copy, Clone)]
pub struct NavigationMenuItemContext {
    pub id: &'static str,
}

/// Access the navigation menu item context.
pub fn use_navigation_menu_item() -> NavigationMenuItemContext {
    use_context::<NavigationMenuItemContext>()
        .expect("use_navigation_menu_item must be called within NavigationMenuItemRoot")
}

/// Root container for a navigation menu.
#[component]
pub fn NavigationMenuRoot(
    #[prop(optional, into)] class: String,
    children: Children,
) -> impl IntoView {
    let ctx = NavigationMenuContext::new();
    provide_context(ctx);

    view! {
        <nav class=class data-component="navigation-menu">
            {children()}
        </nav>
    }
}

/// Horizontal list of navigation menu items.
#[component]
pub fn NavigationMenuList(
    #[prop(optional, into)] class: String,
    children: Children,
) -> impl IntoView {
    view! {
        <ul class=class role="menubar">
            {children()}
        </ul>
    }
}

/// An individual item in the navigation menu. Provides item-level context.
#[component]
pub fn NavigationMenuItemRoot(
    #[prop(into)] id: &'static str,
    #[prop(optional, into)] class: String,
    children: Children,
) -> impl IntoView {
    let item_ctx = NavigationMenuItemContext { id };
    provide_context(item_ctx);

    let menu_ctx = use_navigation_menu();

    view! {
        <li
            class=class
            role="none"
            data-state=move || if menu_ctx.is_open(id) { "open" } else { "closed" }
        >
            {children()}
        </li>
    }
}

/// A trigger button that toggles the content panel for its parent item.
#[component]
pub fn NavigationMenuTrigger(
    #[prop(optional, into)] class: String,
    children: Children,
) -> impl IntoView {
    let menu_ctx = use_navigation_menu();
    let item_ctx = use_navigation_menu_item();
    let id = item_ctx.id;

    view! {
        <button
            type="button"
            role="menuitem"
            class=class
            aria-expanded=move || menu_ctx.is_open(id)
            data-state=move || if menu_ctx.is_open(id) { "open" } else { "closed" }
            on:click=move |_| menu_ctx.toggle(id)
        >
            {children()}
        </button>
    }
}

/// Content panel displayed when its parent item's trigger is activated.
#[component]
pub fn NavigationMenuContent(
    #[prop(optional, into)] class: String,
    children: ChildrenFn,
) -> impl IntoView {
    let menu_ctx = use_navigation_menu();
    let item_ctx = use_navigation_menu_item();
    let id = item_ctx.id;

    view! {
        <div
            class=class
            role="menu"
            hidden=move || !menu_ctx.is_open(id)
            data-state=move || if menu_ctx.is_open(id) { "open" } else { "closed" }
        >
            {children()}
        </div>
    }
    .into_any()
}

/// A styled navigation link.
#[component]
pub fn NavigationMenuLink(
    #[prop(into)] href: String,
    #[prop(optional, into)] class: String,
    children: Children,
) -> impl IntoView {
    view! {
        <a href=href class=class role="menuitem">
            {children()}
        </a>
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing::reactive_scope;

    #[test]
    fn class_prop_accepts_str_and_string() {
        assert_class_prop!(
            NavigationMenuRootProps,
            NavigationMenuListProps,
            NavigationMenuTriggerProps,
            NavigationMenuContentProps,
            NavigationMenuLinkProps,
        );
    }

    #[test]
    fn toggle_opens_and_closes() {
        reactive_scope(|| {
            let ctx = NavigationMenuContext::new();
            assert!(!ctx.is_open("item1"));

            ctx.toggle("item1");
            assert!(ctx.is_open("item1"));

            ctx.toggle("item1");
            assert!(!ctx.is_open("item1"));
        });
    }

    #[test]
    fn opening_one_item_replaces_another() {
        reactive_scope(|| {
            let ctx = NavigationMenuContext::new();
            ctx.open("item1");
            assert!(ctx.is_open("item1"));

            ctx.open("item2");
            assert!(!ctx.is_open("item1"));
            assert!(ctx.is_open("item2"));
        });
    }

    #[test]
    fn close_clears_active_item() {
        reactive_scope(|| {
            let ctx = NavigationMenuContext::new();
            ctx.open("item1");
            ctx.close();
            assert!(!ctx.is_open("item1"));
            assert_eq!(ctx.active_item.get(), None);
        });
    }
}
