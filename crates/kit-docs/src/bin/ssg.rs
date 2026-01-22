//! Static Site Generation (SSG) binary for holt-kit-docs
//!
//! This generates static HTML files for each story route to be served by Docusaurus.
//! Uses Leptos SSR to render the actual components.

// Import stories to register them with inventory
#[allow(unused_imports)]
use holt_kit_docs::stories as _;

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

fn render_navbar() -> &'static str {
    r#"<nav class="kit-navbar">
    <div class="kit-navbar-inner">
        <a href="/" class="kit-navbar-logo">
            <img src="/img/logo.svg" alt="Holt" height="32" />
        </a>
        <div class="kit-navbar-items">
            <a href="/docs/tutorials/" class="kit-navbar-link">Docs</a>
            <a href="/kit/" class="kit-navbar-link kit-navbar-link--active">Kit</a>
        </div>
        <div class="kit-navbar-items kit-navbar-right">
            <a href="https://github.com/aonyx-labs/holt" class="kit-navbar-link" target="_blank" rel="noopener noreferrer">
                GitHub
            </a>
        </div>
    </div>
</nav>"#
}

fn render_route_to_html(route: &str) -> String {
    // Create a new reactive owner for this render
    let owner = Owner::new();
    owner.set();

    // Provide the full URL including base path for the router
    let full_route = format!("{}{}", BASE_PATH, route);
    provide_context(RequestUrl::new(&full_route));

    // Create server meta context - returns (context, output)
    let (meta_context, _meta_output) = ServerMetaContext::new();
    provide_context(meta_context);

    // Render the App component to HTML with base path
    let html = owner.with(|| {
        let app = App(AppProps { base: BASE_PATH });
        // Use Leptos's to_html() method available in SSR mode
        app.to_html()
    });

    owner.cleanup();

    html
}

fn wrap_in_html_document(body: &str, title: &str) -> String {
    format!(
        r#"<!DOCTYPE html>
<html lang="en" dir="ltr" data-theme="light">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{title}</title>
    <link rel="stylesheet" href="/kit/styles.css">
</head>
<body class="kit-body">
{navbar}
<div class="kit-content">
{body}
</div>
</body>
</html>"#,
        navbar = render_navbar(),
        title = title,
        body = body
    )
}

fn main() {
    // Initialize the story registry for SSR
    init_for_ssr();

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
    let index_body = render_route_to_html("/");
    let index_html = wrap_in_html_document(&index_body, "Holt UI Kit");
    let index_path = output_path.join("index.html");
    fs::write(&index_path, index_html).expect("Failed to write index.html");
    println!("  Generated: {}", index_path.display());

    // Generate story pages
    for story in &stories {
        let route = format!("/story/{}", story.id);
        println!("  Rendering: {}", route);

        let story_dir = output_path.join("story").join(story.id);
        fs::create_dir_all(&story_dir).expect("Failed to create story directory");

        let story_body = render_route_to_html(&route);
        let story_html =
            wrap_in_html_document(&story_body, &format!("{} - Holt UI Kit", story.name));
        let story_path = story_dir.join("index.html");
        fs::write(&story_path, story_html).expect("Failed to write story page");
        println!("  Generated: {}", story_path.display());
    }

    // Copy compiled styles.css to output directory
    // First try dist/ (compiled), then fall back to public/ (source)
    let dist_dir = Path::new("crates/kit-docs/dist");
    let styles_dest = output_path.join("styles.css");

    let copied = if dist_dir.exists() {
        // Find the compiled CSS file (has hash in name)
        fs::read_dir(dist_dir)
            .ok()
            .and_then(|entries| {
                entries.filter_map(|e| e.ok()).find(|e| {
                    e.path()
                        .file_name()
                        .and_then(|n| n.to_str())
                        .is_some_and(|n| n.starts_with("styles") && n.ends_with(".css"))
                })
            })
            .map(|entry| {
                fs::copy(entry.path(), &styles_dest).expect("Failed to copy styles.css");
                true
            })
            .unwrap_or(false)
    } else {
        false
    };

    if copied {
        println!("  Copied: {}", styles_dest.display());
    } else {
        eprintln!("  Warning: No compiled styles.css found in dist/. Run `trunk build` first.");
    }

    println!("\nStatic site generation complete!");
    println!("Output directory: {}", output_path.display());
}
