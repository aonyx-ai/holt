//! Static Site Generation (SSG) binary for holt-kit-docs
//!
//! This generates static HTML files for each story route to be served by Docusaurus.
//! Uses Leptos SSR to render the actual components.

#[allow(
    unused_imports,
    reason = "Import stories to register them with inventory"
)]
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

/// Renders a route to a complete HTML page
///
/// The function creates a new reactive owner for this render. It provides the full URL, including
/// the base path, for the router. It creates the server meta context, whose output receives the
/// meta tags from the `leptos_meta` components.
///
/// The function renders the `App` component to HTML. It builds the HTML shell with bare `<html>`
/// and `<body>` tags for `inject_meta_context` to fill in. That call inserts the Leptos-rendered
/// meta elements, html attributes, and body attributes. If Leptos did not set a title, the
/// function injects the fallback title.
#[bon::builder]
async fn render_route_to_html(route: &str, title: &str) -> String {
    let owner = Owner::new();
    owner.set();

    let full_route = format!("{}{}", BASE_PATH, route);
    provide_context(RequestUrl::new(&full_route));

    let (meta_context, meta_output) = ServerMetaContext::new();
    provide_context(meta_context);

    let body_html = owner.with(|| {
        let app = App(AppProps { base: BASE_PATH });
        app.to_html()
    });

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

    let input_stream = futures::stream::iter(std::iter::once(shell));
    let mut output_stream = std::pin::pin!(meta_output.inject_meta_context(input_stream).await);

    let mut html = String::new();
    while let Some(chunk) = output_stream.next().await {
        html.push_str(&chunk);
    }

    if !html.contains("<title>")
        && let Some(pos) = html.find("<!--HEAD-->")
    {
        let insert_at = pos + "<!--HEAD-->".len();
        html.insert_str(insert_at, &format!("<title>{}</title>", title));
    }

    owner.cleanup();

    html
}

/// Generates the static site on a tokio `LocalSet`
///
/// The function initializes the executor so Leptos effects can spawn futures. It initializes the
/// story registry for SSR. Leptos effects use `spawn_local`, which requires a `LocalSet` in tokio.
#[tokio::main]
async fn main() {
    any_spawner::Executor::init_tokio().expect("Failed to initialize tokio executor");

    init_for_ssr();

    let local = tokio::task::LocalSet::new();
    local
        .run_until(async {
            generate_static_site().await;
        })
        .await;
}

/// Generates the static pages and copies the build artifacts
///
/// The function creates the output directory and gets all stories to generate routes. It generates
/// the index page and one page for each story. It copies the trunk build artifacts (CSS, JS, WASM)
/// from `dist/` to the output directory.
async fn generate_static_site() {
    let output_dir = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "docs/static/kit".to_string());

    let output_path = Path::new(&output_dir);

    fs::create_dir_all(output_path).expect("Failed to create output directory");

    let stories = get_all_stories();

    println!("Generating static pages for {} stories...", stories.len());

    println!("  Rendering: /");
    let index_html = render_route_to_html()
        .route("/")
        .title("Holt UI Kit")
        .call()
        .await;
    let index_path = output_path.join("index.html");
    fs::write(&index_path, index_html).expect("Failed to write index.html");
    println!("  Generated: {}", index_path.display());

    for story in &stories {
        let route = format!("/story/{}", story.id());
        println!("  Rendering: {}", route);

        let story_dir = output_path.join("story").join(story.id());
        fs::create_dir_all(&story_dir).expect("Failed to create story directory");

        let story_html = render_route_to_html()
            .route(&route)
            .title(&format!("{} - Holt UI Kit", story.name()))
            .call()
            .await;
        let story_path = story_dir.join("index.html");
        fs::write(&story_path, story_html).expect("Failed to write story page");
        println!("  Generated: {}", story_path.display());
    }

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
