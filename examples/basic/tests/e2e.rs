use std::path::Path;

use doco::{By, Client, Context, Doco, Mount, Result, Server, WaitFor};

static CADDYFILE: &str = include_str!("Caddyfile");

/// Returns the texts of the links in the sidebar navigation
async fn nav_link_texts(client: &Client) -> Result<Vec<String>> {
    client
        .goto("/")
        .await
        .context("failed to open the homepage")?;
    tokio::time::sleep(std::time::Duration::from_millis(500)).await;

    let nav_links = client
        .find_all(By::Css("nav a"))
        .await
        .context("failed to find the navigation links")?;
    futures::future::join_all(nav_links.iter().map(|link| link.text()))
        .await
        .into_iter()
        .collect::<std::result::Result<Vec<_>, _>>()
        .context("failed to read the navigation link texts")
}

/// Opens the story page of Square A and returns the texts of the variant options
async fn variant_option_texts(client: &Client) -> Result<Vec<String>> {
    client
        .goto("/story/square-a")
        .await
        .context("failed to open the story page")?;
    tokio::time::sleep(std::time::Duration::from_millis(500)).await;

    let options = client
        .find_all(By::Css("select option"))
        .await
        .context("failed to find the variant options")?;
    futures::future::join_all(options.iter().map(|o| o.text()))
        .await
        .into_iter()
        .collect::<std::result::Result<Vec<_>, _>>()
        .context("failed to read the variant option texts")
}

/// Opens the story page of Square A and returns the rectangle of the red div
async fn red_div_rect(client: &Client) -> Result<thirtyfour::ElementRect> {
    client
        .goto("/story/square-a")
        .await
        .context("failed to open the story page")?;
    tokio::time::sleep(std::time::Duration::from_millis(500)).await;

    let div = client
        .find(By::Css("div[style*='background:red']"))
        .await
        .context("failed to find the red div")?;
    div.rect()
        .await
        .context("failed to read the red div rectangle")
}

#[doco::test]
async fn homepage_has_story_navigation_lists_square_a(client: Client) -> Result<()> {
    let texts = nav_link_texts(&client)
        .await
        .context("failed to list the navigation links")?;

    assert!(
        texts.iter().any(|t| t.contains("Square A")),
        "missing Square A nav link, found: {:?}",
        texts
    );
    Ok(())
}

#[doco::test]
async fn homepage_has_story_navigation_lists_square_b(client: Client) -> Result<()> {
    let texts = nav_link_texts(&client)
        .await
        .context("failed to list the navigation links")?;

    assert!(
        texts.iter().any(|t| t.contains("Square B")),
        "missing Square B nav link, found: {:?}",
        texts
    );
    Ok(())
}

#[doco::test]
async fn homepage_screenshot(client: Client) -> Result<()> {
    client
        .goto("/")
        .await
        .context("failed to open the homepage")?;
    tokio::time::sleep(std::time::Duration::from_millis(500)).await;

    let screenshot = client
        .screenshot_as_png()
        .await
        .context("failed to take the screenshot")?;
    compare_baseline("homepage", &screenshot).context("homepage differs from the baseline")?;
    Ok(())
}

/// Shows that the story page heading is the name of the story
///
/// The selector starts at `main`, so that it skips the h1 of the sidebar branding.
#[doco::test]
async fn story_page_renders_variant_has_heading(client: Client) -> Result<()> {
    client
        .goto("/story/square-a")
        .await
        .context("failed to open the story page")?;
    tokio::time::sleep(std::time::Duration::from_millis(500)).await;

    let heading = client
        .find(By::Css("main h1"))
        .await
        .context("failed to find the heading")?;

    assert_eq!(
        heading
            .text()
            .await
            .context("failed to read the heading text")?,
        "Square A"
    );
    Ok(())
}

#[doco::test]
async fn story_page_renders_variant_red_div_is_100px_tall(client: Client) -> Result<()> {
    let rect = red_div_rect(&client)
        .await
        .context("failed to measure the red div")?;

    assert!(
        (rect.height - 100.0).abs() < 2.0,
        "red div should be 100px tall, got {}",
        rect.height
    );
    Ok(())
}

#[doco::test]
async fn story_page_renders_variant_red_div_is_100px_wide(client: Client) -> Result<()> {
    let rect = red_div_rect(&client)
        .await
        .context("failed to measure the red div")?;

    assert!(
        (rect.width - 100.0).abs() < 2.0,
        "red div should be 100px wide, got {}",
        rect.width
    );
    Ok(())
}

#[doco::test]
async fn story_page_renders_variant_selector_has_blue(client: Client) -> Result<()> {
    let option_texts = variant_option_texts(&client)
        .await
        .context("failed to list the variant options")?;

    assert!(option_texts.contains(&"Blue".to_string()));
    Ok(())
}

#[doco::test]
async fn story_page_renders_variant_selector_has_red(client: Client) -> Result<()> {
    let option_texts = variant_option_texts(&client)
        .await
        .context("failed to list the variant options")?;

    assert!(option_texts.contains(&"Red".to_string()));
    Ok(())
}

#[doco::test]
async fn story_page_screenshot(client: Client) -> Result<()> {
    client
        .goto("/story/square-a")
        .await
        .context("failed to open the story page")?;
    tokio::time::sleep(std::time::Duration::from_millis(500)).await;

    let screenshot = client
        .screenshot_as_png()
        .await
        .context("failed to take the screenshot")?;
    compare_baseline("story-square-a", &screenshot)
        .context("story page differs from the baseline")?;
    Ok(())
}

/// Compares a screenshot against the saved baseline
///
/// When `SAVE_BASELINES` is set, the function writes new baselines instead of comparing. A
/// mismatch writes the actual screenshot next to the baseline for debugging.
fn compare_baseline(name: &str, screenshot: &[u8]) -> Result<()> {
    let baseline_path = Path::new("tests/e2e-baselines").join(format!("{name}.png"));

    if std::env::var("SAVE_BASELINES").is_ok() {
        std::fs::create_dir_all(baseline_path.parent().unwrap())
            .context("failed to create the baseline directory")?;
        std::fs::write(&baseline_path, screenshot).context("failed to write the baseline")?;
        println!("  saved baseline: {}", baseline_path.display());
        return Ok(());
    }

    let baseline = std::fs::read(&baseline_path)
        .map_err(|_| {
            doco::anyhow!(
                "baseline not found: {} (run with SAVE_BASELINES=1 to generate)",
                baseline_path.display()
            )
        })
        .context("failed to read the baseline")?;

    if baseline != screenshot {
        let actual_path = baseline_path.with_extension("actual.png");
        std::fs::write(&actual_path, screenshot).ok();
        return Err(doco::anyhow!(
            "screenshot mismatch for {name} (actual written to {})",
            actual_path.display()
        ));
    }

    Ok(())
}

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
