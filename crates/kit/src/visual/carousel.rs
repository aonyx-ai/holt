use leptos::prelude::*;
use tailwind_fuse::*;

use crate::behavior::{
    CarouselContent as CarouselContentPrimitive, CarouselItem as CarouselItemPrimitive,
    CarouselNext as CarouselNextPrimitive, CarouselOrientation,
    CarouselPrevious as CarouselPreviousPrimitive, CarouselRoot as CarouselRootPrimitive,
};

#[derive(TwClass)]
#[tw(class = "relative")]
struct CarouselStyle {}

#[derive(TwClass)]
#[tw(class = "flex overflow-x-auto scroll-smooth snap-x snap-mandatory -ml-4")]
struct CarouselContentHorizontalStyle {}

#[derive(TwClass)]
#[tw(class = "flex flex-col overflow-y-auto scroll-smooth snap-y snap-mandatory -mt-4")]
struct CarouselContentVerticalStyle {}

#[derive(TwClass)]
#[tw(class = "min-w-0 shrink-0 grow-0 snap-start pl-4")]
struct CarouselItemHorizontalStyle {}

#[derive(TwClass)]
#[tw(class = "min-h-0 shrink-0 grow-0 snap-start pt-4")]
struct CarouselItemVerticalStyle {}

#[derive(TwClass)]
#[tw(
    class = "absolute h-8 w-8 rounded-full inline-flex items-center justify-center border bg-background hover:bg-accent hover:text-accent-foreground disabled:opacity-50"
)]
struct CarouselNavStyle {}

#[derive(TwClass)]
#[tw(class = "-left-12 top-1/2 -translate-y-1/2")]
struct CarouselPrevHorizontalPos {}

#[derive(TwClass)]
#[tw(class = "-top-12 left-1/2 -translate-x-1/2")]
struct CarouselPrevVerticalPos {}

#[derive(TwClass)]
#[tw(class = "-right-12 top-1/2 -translate-y-1/2")]
struct CarouselNextHorizontalPos {}

#[derive(TwClass)]
#[tw(class = "-bottom-12 left-1/2 -translate-x-1/2")]
struct CarouselNextVerticalPos {}

/// Styled carousel root.
#[component]
pub fn Carousel(
    #[prop(optional, into)] class: String,
    #[prop(optional)] orientation: CarouselOrientation,
    children: Children,
) -> impl IntoView {
    let class = CarouselStyle {}.with_class(&class);
    view! {
        <CarouselRootPrimitive class=class orientation=orientation>
            {children()}
        </CarouselRootPrimitive>
    }
}

/// Styled scrollable container for carousel items.
#[component]
pub fn CarouselContent(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    let ctx = crate::behavior::use_carousel();
    let class = match ctx.orientation {
        CarouselOrientation::Horizontal => CarouselContentHorizontalStyle {}.with_class(&class),
        CarouselOrientation::Vertical => CarouselContentVerticalStyle {}.with_class(&class),
    };
    view! {
        <CarouselContentPrimitive class=class>
            {children()}
        </CarouselContentPrimitive>
    }
}

/// Styled individual slide.
#[component]
pub fn CarouselItem(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    let ctx = crate::behavior::use_carousel();
    let class = match ctx.orientation {
        CarouselOrientation::Horizontal => CarouselItemHorizontalStyle {}.with_class(&class),
        CarouselOrientation::Vertical => CarouselItemVerticalStyle {}.with_class(&class),
    };
    view! {
        <CarouselItemPrimitive class=class>
            {children()}
        </CarouselItemPrimitive>
    }
}

/// Styled previous-slide button with a left-arrow icon.
#[component]
pub fn CarouselPrevious(#[prop(optional, into)] class: String) -> impl IntoView {
    let ctx = crate::behavior::use_carousel();
    let pos_class = match ctx.orientation {
        CarouselOrientation::Horizontal => CarouselPrevHorizontalPos {}.to_class(),
        CarouselOrientation::Vertical => CarouselPrevVerticalPos {}.to_class(),
    };
    let class = CarouselNavStyle {}
        .with_class(&pos_class)
        .with_class(&class);
    view! {
        <CarouselPreviousPrimitive class=class>
            <leptos_icons::Icon icon=icondata::LuArrowLeft attr:class="h-4 w-4" />
            <span class="sr-only">Previous slide</span>
        </CarouselPreviousPrimitive>
    }
}

/// Styled next-slide button with a right-arrow icon.
#[component]
pub fn CarouselNext(#[prop(optional, into)] class: String) -> impl IntoView {
    let ctx = crate::behavior::use_carousel();
    let pos_class = match ctx.orientation {
        CarouselOrientation::Horizontal => CarouselNextHorizontalPos {}.to_class(),
        CarouselOrientation::Vertical => CarouselNextVerticalPos {}.to_class(),
    };
    let class = CarouselNavStyle {}
        .with_class(&pos_class)
        .with_class(&class);
    view! {
        <CarouselNextPrimitive class=class>
            <leptos_icons::Icon icon=icondata::LuArrowRight attr:class="h-4 w-4" />
            <span class="sr-only">Next slide</span>
        </CarouselNextPrimitive>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn class_prop_accepts_str_and_string() {
        assert_class_prop!(
            CarouselProps,
            CarouselContentProps,
            CarouselItemProps,
            CarouselPreviousProps,
            CarouselNextProps,
        );
    }
}
