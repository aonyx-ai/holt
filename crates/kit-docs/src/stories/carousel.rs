// @component Carousel
#[cfg(feature = "e2e")]
mod e2e;

use holt_book::{story, variant};
use holt_kit::visual::{
    Card, CardContent, Carousel, CarouselContent, CarouselItem, CarouselNext, CarouselPrevious,
};
use leptos::prelude::*;

#[variant]
fn basic() -> AnyView {
    view! {
        <div class="w-full max-w-xs mx-auto px-12">
            <Carousel>
                <CarouselContent>
                    <CarouselItem class="basis-full">
                        <Card>
                            <CardContent class="flex aspect-square items-center justify-center p-6">
                                <span class="text-4xl font-semibold">1</span>
                            </CardContent>
                        </Card>
                    </CarouselItem>
                    <CarouselItem class="basis-full">
                        <Card>
                            <CardContent class="flex aspect-square items-center justify-center p-6">
                                <span class="text-4xl font-semibold">2</span>
                            </CardContent>
                        </Card>
                    </CarouselItem>
                    <CarouselItem class="basis-full">
                        <Card>
                            <CardContent class="flex aspect-square items-center justify-center p-6">
                                <span class="text-4xl font-semibold">3</span>
                            </CardContent>
                        </Card>
                    </CarouselItem>
                </CarouselContent>
                <CarouselPrevious />
                <CarouselNext />
            </Carousel>
        </div>
    }
    .into_any()
}

#[variant]
fn multiple_items() -> AnyView {
    view! {
        <div class="w-full max-w-sm mx-auto px-12">
            <Carousel>
                <CarouselContent>
                    <CarouselItem class="basis-1/3">
                        <Card>
                            <CardContent class="flex aspect-square items-center justify-center p-4">
                                <span class="text-2xl font-semibold">1</span>
                            </CardContent>
                        </Card>
                    </CarouselItem>
                    <CarouselItem class="basis-1/3">
                        <Card>
                            <CardContent class="flex aspect-square items-center justify-center p-4">
                                <span class="text-2xl font-semibold">2</span>
                            </CardContent>
                        </Card>
                    </CarouselItem>
                    <CarouselItem class="basis-1/3">
                        <Card>
                            <CardContent class="flex aspect-square items-center justify-center p-4">
                                <span class="text-2xl font-semibold">3</span>
                            </CardContent>
                        </Card>
                    </CarouselItem>
                    <CarouselItem class="basis-1/3">
                        <Card>
                            <CardContent class="flex aspect-square items-center justify-center p-4">
                                <span class="text-2xl font-semibold">4</span>
                            </CardContent>
                        </Card>
                    </CarouselItem>
                    <CarouselItem class="basis-1/3">
                        <Card>
                            <CardContent class="flex aspect-square items-center justify-center p-4">
                                <span class="text-2xl font-semibold">5</span>
                            </CardContent>
                        </Card>
                    </CarouselItem>
                </CarouselContent>
                <CarouselPrevious />
                <CarouselNext />
            </Carousel>
        </div>
    }
    .into_any()
}

#[variant]
fn no_navigation() -> AnyView {
    view! {
        <div class="w-full max-w-xs mx-auto">
            <Carousel>
                <CarouselContent>
                    <CarouselItem class="basis-full">
                        <Card>
                            <CardContent class="flex aspect-video items-center justify-center p-6">
                                <span class="text-lg text-muted-foreground">Swipe to navigate</span>
                            </CardContent>
                        </Card>
                    </CarouselItem>
                    <CarouselItem class="basis-full">
                        <Card>
                            <CardContent class="flex aspect-video items-center justify-center p-6">
                                <span class="text-lg text-muted-foreground">Second slide</span>
                            </CardContent>
                        </Card>
                    </CarouselItem>
                    <CarouselItem class="basis-full">
                        <Card>
                            <CardContent class="flex aspect-video items-center justify-center p-6">
                                <span class="text-lg text-muted-foreground">Third slide</span>
                            </CardContent>
                        </Card>
                    </CarouselItem>
                </CarouselContent>
            </Carousel>
        </div>
    }
    .into_any()
}

include!(concat!(env!("OUT_DIR"), "/stories/carousel_source.rs"));

#[story(id = "carousel", name = "Carousel", extra_docs = CAROUSEL_SOURCE)]
/// A carousel with smooth scrolling and snap-to-item behavior
const CAROUSEL_STORY: () = &[basic, multiple_items, no_navigation];
