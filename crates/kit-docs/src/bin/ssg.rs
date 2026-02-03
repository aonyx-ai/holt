//! Static Site Generation (SSG) binary for holt-kit-docs
//!
//! This generates static HTML files for each story route to be served by Docusaurus.
//! Uses Leptos SSR to render the actual components.

// Import stories to register them with inventory
#[allow(unused_imports)]
use holt_kit_docs::stories as _;

use futures::StreamExt;
use holt_book::{App, AppProps, Story, init_for_ssr};
use leptos::prelude::*;
use leptos_meta::ServerMetaContext;
use leptos_router::location::RequestUrl;
use std::fs;
use std::path::Path;

fn get_all_stories() -> Vec<&'static Story> {
    inventory::iter::<&'static Story>
        .into_iter()
        .copied()
        .collect()
}

/// Base path for the kit documentation (used when serving from /kit/ in Docusaurus)
const BASE_PATH: &str = "/kit";

async fn render_route_to_html(route: &str, title: &str) -> String {
    // Create a new reactive owner for this render
    let owner = Owner::new();
    owner.set();

    // Provide the full URL including base path for the router
    let full_route = format!("{}{}", BASE_PATH, route);
    provide_context(RequestUrl::new(&full_route));

    // Create server meta context — the output receives meta tags from leptos_meta components
    let (meta_context, meta_output) = ServerMetaContext::new();
    provide_context(meta_context);

    // Render the App component to HTML
    let body_html = owner.with(|| {
        let app = App(AppProps { base: BASE_PATH });
        app.to_html()
    });

    // Build the HTML shell with bare <html> and <body> tags for inject_meta_context to fill in
    let shell = format!(
        r#"<!DOCTYPE html>
<html>
<head>
    <!--HEAD-->
    <link rel="stylesheet" href="/kit/styles.css">
    <link rel="preload" href="/kit/holt-kit-docs_bg.wasm" as="fetch" crossorigin>
    <script type="module">
      import init from '/kit/holt-kit-docs.js';
      init('/kit/holt-kit-docs_bg.wasm');
    </script>
</head>
<body class="kit-body">{body_html}</body>
</html>"#,
        body_html = body_html,
    );

    // Use inject_meta_context to insert Leptos-rendered meta elements, html attrs, body attrs
    let input_stream = futures::stream::iter(std::iter::once(shell));
    let mut output_stream = std::pin::pin!(meta_output.inject_meta_context(input_stream).await);

    let mut html = String::new();
    while let Some(chunk) = output_stream.next().await {
        html.push_str(&chunk);
    }

    // If Leptos didn't set a title, inject our fallback
    if !html.contains("<title>")
        && let Some(pos) = html.find("<!--HEAD-->")
    {
        let insert_at = pos + "<!--HEAD-->".len();
        html.insert_str(insert_at, &format!("<title>{}</title>", title));
    }

    owner.cleanup();

    html
}

#[tokio::main]
async fn main() {
    // Initialize the executor so Leptos effects can spawn futures
    any_spawner::Executor::init_tokio().expect("Failed to initialize tokio executor");

    // Initialize the story registry for SSR
    init_for_ssr();

    // Leptos effects use spawn_local which requires a LocalSet in tokio
    let local = tokio::task::LocalSet::new();
    local
        .run_until(async {
            generate_static_site().await;
        })
        .await;
}

async fn generate_static_site() {
    let output_dir = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "docs/static/kit".to_string());

    let output_path = Path::new(&output_dir);

    // Create output directory
    fs::create_dir_all(output_path).expect("Failed to create output directory");

    // Get all stories to generate routes
    let stories = get_all_stories();

    println!("Generating static pages for {} stories...", stories.len());

    // Generate index page
    println!("  Rendering: /");
    let index_html = render_route_to_html("/", "Holt UI Kit").await;
    let index_path = output_path.join("index.html");
    fs::write(&index_path, index_html).expect("Failed to write index.html");
    println!("  Generated: {}", index_path.display());

    // Generate story pages
    for story in &stories {
        let route = format!("/story/{}", story.id);
        println!("  Rendering: {}", route);

        let story_dir = output_path.join("story").join(story.id);
        fs::create_dir_all(&story_dir).expect("Failed to create story directory");

        let story_html =
            render_route_to_html(&route, &format!("{} - Holt UI Kit", story.name)).await;
        let story_path = story_dir.join("index.html");
        fs::write(&story_path, story_html).expect("Failed to write story page");
        println!("  Generated: {}", story_path.display());
    }

    // Copy trunk build artifacts (CSS, JS, WASM) from dist/ to output directory
    let dist_dir = Path::new("crates/kit-docs/dist");
    let assets = ["styles.css", "holt-kit-docs.js", "holt-kit-docs_bg.wasm"];

    if dist_dir.exists() {
        for asset in &assets {
            let src = dist_dir.join(asset);
            let dest = output_path.join(asset);
            if src.exists() {
                fs::copy(&src, &dest).unwrap_or_else(|e| panic!("Failed to copy {}: {}", asset, e));
                println!("  Copied: {}", dest.display());
            } else {
                eprintln!("  Warning: {} not found in dist/", asset);
            }
        }
    } else {
        eprintln!(
            "  Warning: dist/ not found. Run `trunk build --release --filehash false --no-default-features --features hydrate` first."
        );
    }

    println!("\nStatic site generation complete!");
    println!("Output directory: {}", output_path.display());
}
