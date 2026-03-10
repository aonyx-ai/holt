use std::path::Path;

use doco::{Client, Doco, Mount, Result, Server, WaitFor};
use thirtyfour::By;

static CADDYFILE: &str = include_str!("Caddyfile");

// --- DOM tests ---

#[doco::test]
async fn homepage_has_story_navigation(client: Client) -> Result<()> {
    client.goto("/").await?;
    tokio::time::sleep(std::time::Duration::from_millis(500)).await;

    // Sidebar should list both stories
    let nav_links = client.find_all(By::Css("nav a")).await?;
    let texts: Vec<String> = futures::future::join_all(nav_links.iter().map(|link| link.text()))
        .await
        .into_iter()
        .collect::<std::result::Result<Vec<_>, _>>()?;

    assert!(
        texts.iter().any(|t| t.contains("Square A")),
        "missing Square A nav link, found: {:?}",
        texts
    );
    assert!(
        texts.iter().any(|t| t.contains("Square B")),
        "missing Square B nav link, found: {:?}",
        texts
    );
    Ok(())
}

#[doco::test]
async fn story_page_renders_variant(client: Client) -> Result<()> {
    client.goto("/story/square-a").await?;
    tokio::time::sleep(std::time::Duration::from_millis(500)).await;

    // Story heading (use main to skip sidebar branding h1)
    let heading = client.find(By::Css("main h1")).await?;
    assert_eq!(heading.text().await?, "Square A");

    // Variant selector has Red and Blue options
    let options = client.find_all(By::Css("select option")).await?;
    let option_texts: Vec<String> = futures::future::join_all(options.iter().map(|o| o.text()))
        .await
        .into_iter()
        .collect::<std::result::Result<Vec<_>, _>>()?;
    assert!(option_texts.contains(&"Red".to_string()));
    assert!(option_texts.contains(&"Blue".to_string()));

    // Preview area contains the red div (default variant)
    let div = client.find(By::Css("div[style*='background:red']")).await?;
    let rect = div.rect().await?;
    assert!(
        (rect.width - 100.0).abs() < 2.0,
        "red div should be 100px wide, got {}",
        rect.width
    );
    assert!(
        (rect.height - 100.0).abs() < 2.0,
        "red div should be 100px tall, got {}",
        rect.height
    );

    Ok(())
}

// --- Visual regression tests ---

#[doco::test]
async fn homepage_screenshot(client: Client) -> Result<()> {
    client.goto("/").await?;
    tokio::time::sleep(std::time::Duration::from_millis(500)).await;

    let screenshot = client.screenshot_as_png().await?;
    compare_baseline("homepage", &screenshot)?;
    Ok(())
}

#[doco::test]
async fn story_page_screenshot(client: Client) -> Result<()> {
    client.goto("/story/square-a").await?;
    tokio::time::sleep(std::time::Duration::from_millis(500)).await;

    let screenshot = client.screenshot_as_png().await?;
    compare_baseline("story-square-a", &screenshot)?;
    Ok(())
}

// --- Helpers ---

fn compare_baseline(name: &str, screenshot: &[u8]) -> Result<()> {
    let baseline_path = Path::new("tests/e2e-baselines").join(format!("{name}.png"));

    if std::env::var("SAVE_BASELINES").is_ok() {
        // When SAVE_BASELINES is set, write new baselines instead of comparing
        std::fs::create_dir_all(baseline_path.parent().unwrap())?;
        std::fs::write(&baseline_path, screenshot)?;
        println!("  saved baseline: {}", baseline_path.display());
        return Ok(());
    }

    let baseline = std::fs::read(&baseline_path).map_err(|_| {
        doco::anyhow!(
            "baseline not found: {} (run with SAVE_BASELINES=1 to generate)",
            baseline_path.display()
        )
    })?;

    if baseline != screenshot {
        // Write the actual screenshot for debugging
        let actual_path = baseline_path.with_extension("actual.png");
        std::fs::write(&actual_path, screenshot).ok();
        return Err(doco::anyhow!(
            "screenshot mismatch for {name} (actual written to {})",
            actual_path.display()
        ));
    }

    Ok(())
}

// --- Doco configuration ---

#[doco::main]
async fn main() -> Doco {
    let dist = Path::new("dist")
        .canonicalize()
        .expect("dist/ not found — run `trunk build --release` first");
    let dist_str = dist.to_str().expect("dist path not UTF-8");

    let caddyfile_path = std::env::temp_dir().join("holt-e2e-Caddyfile");
    std::fs::write(&caddyfile_path, CADDYFILE).expect("failed to write Caddyfile");
    let caddyfile_str = caddyfile_path.to_str().expect("path not UTF-8").to_string();

    let server = Server::builder()
        .image("caddy")
        .tag("alpine")
        .port(80)
        .wait(WaitFor::message_on_stderr("server running"))
        .mount(Mount::bind_mount(dist_str, "/srv"))
        .mount(Mount::bind_mount(&caddyfile_str, "/etc/caddy/Caddyfile"))
        .build();

    Doco::builder()
        .server(server)
        .headless(true)
        .viewport(doco::Viewport::new(1280, 720))
        .build()
}
