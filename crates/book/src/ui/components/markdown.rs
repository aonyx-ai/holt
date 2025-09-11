use leptos::prelude::*;
use markdown::ParseOptions;

#[component]
fn ExpandableCodeBlock(code: String, lang: Option<String>) -> impl IntoView {
    let (is_expanded, set_is_expanded) = signal(false);

    view! {
        <div class="relative">
            <div
                class=move || {
                    format!(
                        "overflow-hidden transition-all duration-300 bg-muted/30 rounded-lg border {}",
                        if is_expanded.get() {
                            "max-h-none"
                        } else {
                            "max-h-48 cursor-pointer hover:bg-muted/40"
                        },
                    )
                }
                on:click=move |_| {
                    if !is_expanded.get() {
                        set_is_expanded.set(true)
                    }
                }
            >
                <pre class="overflow-x-auto text-sm p-4 whitespace-pre-wrap break-words">
                    <code class=lang.clone()>{code.clone()}</code>
                </pre>
            </div>

            {move || {
                if !is_expanded.get() {
                    view! {
                        <div class="absolute bottom-0 left-0 right-0 h-16 bg-gradient-to-t from-muted/60 via-muted/40 to-transparent rounded-b-lg pointer-events-none"></div>
                        <div class="absolute bottom-4 left-1/2 transform -translate-x-1/2 bg-background/95 border rounded-full px-3 py-1 text-xs font-medium shadow-md pointer-events-none">
                            "Click to expand"
                        </div>
                    }
                        .into_any()
                } else {
                    view! {
                        <button
                            class="absolute bottom-2 right-2 bg-background/90 hover:bg-background border rounded px-2 py-1 text-xs font-medium shadow-sm transition-colors"
                            on:click=move |_| set_is_expanded.set(false)
                        >
                            "Collapse"
                        </button>
                    }
                        .into_any()
                }
            }}
        </div>
    }
}

fn render_node(node: markdown::mdast::Node) -> impl IntoView {
    match node {
        markdown::mdast::Node::Blockquote(blockquote) => view! {
            <blockquote>
                {blockquote.children.iter().map(|child| render_node(child.clone())).collect_view()}
            </blockquote>
        }.into_any(),
        markdown::mdast::Node::Break(_) => view! { <br /> }.into_any(),
        markdown::mdast::Node::Code(code) => view! { <ExpandableCodeBlock code=code.value.clone() lang=code.lang.clone() /> }.into_any(),
        markdown::mdast::Node::Definition(_) => view! { <span /> }.into_any(),
        markdown::mdast::Node::Delete(delete) => view! { <del>{delete.children.iter().map(|child| render_node(child.clone())).collect_view()}</del> }.into_any(),
        markdown::mdast::Node::Emphasis(emphasis) => view! { <em>{emphasis.children.iter().map(|child| render_node(child.clone())).collect_view()}</em> }.into_any(),
        markdown::mdast::Node::Heading(heading) => {
            let classes = match heading.depth {
                1 => "text-2xl font-bold mb-4 mt-6",
                2 => "text-xl font-semibold mb-3 mt-5",
                3 => "text-lg font-medium mb-2 mt-4",
                4 => "text-base font-medium mb-2 mt-3",
                5 => "text-sm font-medium mb-1 mt-2",
                _ => "text-xs font-medium mb-1 mt-2",
            };
            let tag = format!("h{}", heading.depth);
            view! {
                <div class=format!(
                    "{tag} {classes}",
                )>
                    {heading.children.iter().map(|child| render_node(child.clone())).collect_view()}
                </div>
            }.into_any()
        },
        markdown::mdast::Node::Image(image) => view! { <img src=image.url alt=image.alt title=image.title class="max-w-full h-auto rounded" /> }.into_any(),
        markdown::mdast::Node::ImageReference(_) => view! { <span /> }.into_any(),
        markdown::mdast::Node::InlineCode(code) => view! { <code class="bg-muted/50 px-1 py-0.5 rounded text-sm font-mono">{code.value}</code> }.into_any(),
        markdown::mdast::Node::InlineMath(math) => view! { <span class="math-inline">{math.value}</span> }.into_any(),
        markdown::mdast::Node::Link(link) => view! {
            <a href=link.url title=link.title class="text-primary underline hover:no-underline">
                {link.children.iter().map(|child| render_node(child.clone())).collect_view()}
            </a>
        }.into_any(),
        markdown::mdast::Node::LinkReference(_) => view! { <span /> }.into_any(),
        markdown::mdast::Node::List(list) => {
            if list.ordered {
                view! {
                    <ol class="list-decimal list-inside space-y-1 ml-4 mb-4">
                        {list
                            .children
                            .iter()
                            .map(|child| render_node(child.clone()))
                            .collect_view()}
                    </ol>
                }.into_any()
            } else {
                view! {
                    <ul class="list-disc list-inside space-y-1 ml-4 mb-4">
                        {list
                            .children
                            .iter()
                            .map(|child| render_node(child.clone()))
                            .collect_view()}
                    </ul>
                }.into_any()
            }
        },
        markdown::mdast::Node::ListItem(item) => view! {
            <li class="mb-1">
                {item.children.iter().map(|child| render_node(child.clone())).collect_view()}
            </li>
        }.into_any(),
        markdown::mdast::Node::Math(math) => view! { <div class="math-block">{math.value}</div> }.into_any(),
        markdown::mdast::Node::Paragraph(paragraph) => view! {
            <p class="mb-4 leading-relaxed break-words">
                {paragraph.children.iter().map(|child| render_node(child.clone())).collect_view()}
            </p>
        }.into_any(),
        markdown::mdast::Node::Root(root) => root.children.iter().map(|child| render_node(child.clone())).collect_view().into_any(),
        markdown::mdast::Node::Strong(strong) => view! {
            <strong>
                {strong.children.iter().map(|child| render_node(child.clone())).collect_view()}
            </strong>
        }.into_any(),
        markdown::mdast::Node::Text(text) => view! { {text.value} }.into_any(),
        _ => view! { <span /> }.into_any()
    }
}

#[component]
pub(crate) fn Markdown(content: String) -> impl IntoView {
    let ast = markdown::to_mdast(&content, &ParseOptions::default()).unwrap();

    view! {
        <div class="prose prose-sm max-w-none overflow-hidden break-words">{render_node(ast)}</div>
    }
}
