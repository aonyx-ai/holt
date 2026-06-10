// @component Accordion
#[cfg(feature = "e2e")]
mod e2e;

use holt_book::{story, variant};
use holt_kit::visual::{Accordion, AccordionContent, AccordionItem, AccordionTrigger};
use leptos::prelude::*;

#[variant]
fn default() -> AnyView {
    view! {
        <Accordion>
            <AccordionItem value="item-1">
                <AccordionTrigger>"Is it accessible?"</AccordionTrigger>
                <AccordionContent>
                    "Yes. It adheres to the WAI-ARIA design pattern."
                </AccordionContent>
            </AccordionItem>
            <AccordionItem value="item-2">
                <AccordionTrigger>"Is it styled?"</AccordionTrigger>
                <AccordionContent>
                    "Yes. It comes with default styles that match the other components' aesthetic."
                </AccordionContent>
            </AccordionItem>
            <AccordionItem value="item-3">
                <AccordionTrigger>"Is it animated?"</AccordionTrigger>
                <AccordionContent>
                    "Yes. It's animated by default, but you can disable it if you prefer."
                </AccordionContent>
            </AccordionItem>
        </Accordion>
    }
    .into_any()
}

#[variant]
fn multiple() -> AnyView {
    view! {
        <Accordion accordion_type=holt_kit::behavior::AccordionType::Multiple>
            <AccordionItem value="item-1">
                <AccordionTrigger>"First item"</AccordionTrigger>
                <AccordionContent>"Content for the first item."</AccordionContent>
            </AccordionItem>
            <AccordionItem value="item-2">
                <AccordionTrigger>"Second item"</AccordionTrigger>
                <AccordionContent>"Content for the second item."</AccordionContent>
            </AccordionItem>
            <AccordionItem value="item-3">
                <AccordionTrigger>"Third item"</AccordionTrigger>
                <AccordionContent>"Content for the third item."</AccordionContent>
            </AccordionItem>
        </Accordion>
    }
    .into_any()
}

#[variant]
fn default_open() -> AnyView {
    view! {
        <Accordion default_value=vec!["item-2".to_string()]>
            <AccordionItem value="item-1">
                <AccordionTrigger>"First item"</AccordionTrigger>
                <AccordionContent>"Content for the first item."</AccordionContent>
            </AccordionItem>
            <AccordionItem value="item-2">
                <AccordionTrigger>"Second item (open by default)"</AccordionTrigger>
                <AccordionContent>"This item is open by default."</AccordionContent>
            </AccordionItem>
        </Accordion>
    }
    .into_any()
}

include!(concat!(env!("OUT_DIR"), "/stories/accordion_source.rs"));

#[story(id = "accordion", name = "Accordion", extra_docs = ACCORDION_SOURCE)]
/// A vertically stacked set of interactive headings that each reveal a section of content
const ACCORDION_STORY: () = &[default, multiple, default_open];
