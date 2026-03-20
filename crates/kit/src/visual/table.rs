use leptos::children::Children;
use leptos::prelude::*;
use tailwind_fuse::tw_merge;

#[component]
pub fn Table(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    let classes = tw_merge!("w-full caption-bottom text-sm", &class);

    view! {
        <div class="relative w-full overflow-auto">
            <table class=classes>{children()}</table>
        </div>
    }
}

#[component]
pub fn TableHeader(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    let classes = tw_merge!("[&_tr]:border-b", &class);

    view! { <thead class=classes>{children()}</thead> }
}

#[component]
pub fn TableBody(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    let classes = tw_merge!("[&_tr:last-child]:border-0", &class);

    view! { <tbody class=classes>{children()}</tbody> }
}

#[component]
pub fn TableFooter(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    let classes = tw_merge!(
        "border-t bg-muted/50 font-medium [&>tr]:last:border-b-0",
        &class
    );

    view! { <tfoot class=classes>{children()}</tfoot> }
}

#[component]
pub fn TableRow(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    let classes = tw_merge!(
        "border-b transition-colors hover:bg-muted/50 data-[state=selected]:bg-muted",
        &class
    );

    view! { <tr class=classes>{children()}</tr> }
}

#[component]
pub fn TableHead(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    let classes = tw_merge!(
        "h-10 px-2 text-left align-middle font-medium text-muted-foreground",
        &class
    );

    view! { <th class=classes>{children()}</th> }
}

#[component]
pub fn TableCell(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    let classes = tw_merge!("p-2 align-middle", &class);

    view! { <td class=classes>{children()}</td> }
}

#[component]
pub fn TableCaption(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    let classes = tw_merge!("mt-4 text-sm text-muted-foreground", &class);

    view! { <caption class=classes>{children()}</caption> }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn class_prop_accepts_str_and_string() {
        assert_class_prop!(
            TableProps,
            TableHeaderProps,
            TableBodyProps,
            TableFooterProps,
            TableRowProps,
            TableHeadProps,
            TableCellProps,
            TableCaptionProps,
        );
    }
}
