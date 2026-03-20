use leptos::html;
use leptos::prelude::*;

/// Orientation of the carousel scroll axis.
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum CarouselOrientation {
    #[default]
    Horizontal,
    Vertical,
}

/// Shared state for all carousel sub-components.
#[derive(Clone)]
pub struct CarouselContext {
    pub orientation: CarouselOrientation,
    pub container_ref: NodeRef<html::Div>,
}

impl CarouselContext {
    /// Scroll the container backward by one visible page.
    pub fn scroll_prev(&self) {
        let Some(el) = self.container_ref.get() else {
            return;
        };
        let el: &web_sys::HtmlElement = &el;
        match self.orientation {
            CarouselOrientation::Horizontal => {
                let amount = -(el.client_width() as f64);
                el.scroll_by_with_x_and_y(amount, 0.0);
            }
            CarouselOrientation::Vertical => {
                let amount = -(el.client_height() as f64);
                el.scroll_by_with_x_and_y(0.0, amount);
            }
        }
    }

    /// Scroll the container forward by one visible page.
    pub fn scroll_next(&self) {
        let Some(el) = self.container_ref.get() else {
            return;
        };
        let el: &web_sys::HtmlElement = &el;
        match self.orientation {
            CarouselOrientation::Horizontal => {
                let amount = el.client_width() as f64;
                el.scroll_by_with_x_and_y(amount, 0.0);
            }
            CarouselOrientation::Vertical => {
                let amount = el.client_height() as f64;
                el.scroll_by_with_x_and_y(0.0, amount);
            }
        }
    }
}

/// Root carousel primitive that provides context to children.
#[component]
pub fn CarouselRoot(
    #[prop(optional, into)] class: String,
    #[prop(optional)] orientation: CarouselOrientation,
    children: Children,
) -> impl IntoView {
    let container_ref = NodeRef::new();
    let ctx = CarouselContext {
        orientation,
        container_ref,
    };
    provide_context(ctx);

    let role = "region";
    let aria_roledescription = "carousel";

    view! {
        <div class=class role=role aria-roledescription=aria_roledescription>
            {children()}
        </div>
    }
}

/// Access the carousel context from a child component.
pub fn use_carousel() -> CarouselContext {
    use_context::<CarouselContext>().expect("use_carousel must be called within CarouselRoot")
}

/// Scrollable container that holds carousel items. Uses CSS scroll-snap.
#[component]
pub fn CarouselContent(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    let ctx = use_carousel();

    view! {
        <div class=class node_ref=ctx.container_ref role="group" aria-roledescription="slide-group">
            {children()}
        </div>
    }
}

/// A single slide in the carousel.
#[component]
pub fn CarouselItem(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    view! {
        <div class=class role="group" aria-roledescription="slide">
            {children()}
        </div>
    }
}

/// Button that scrolls to the previous page.
#[component]
pub fn CarouselPrevious(
    #[prop(optional, into)] class: String,
    children: Children,
) -> impl IntoView {
    let ctx = use_carousel();

    view! {
        <button
            type="button"
            class=class
            on:click=move |_| ctx.scroll_prev()
            aria-label="Previous slide"
        >
            {children()}
        </button>
    }
}

/// Button that scrolls to the next page.
#[component]
pub fn CarouselNext(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    let ctx = use_carousel();

    view! {
        <button
            type="button"
            class=class
            on:click=move |_| ctx.scroll_next()
            aria-label="Next slide"
        >
            {children()}
        </button>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn class_prop_accepts_str_and_string() {
        assert_class_prop!(
            CarouselRootProps,
            CarouselContentProps,
            CarouselItemProps,
            CarouselPreviousProps,
            CarouselNextProps,
        );
    }
}
