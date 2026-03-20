use doco::{Client, Result};
use thirtyfour::By;

/// Aspect ratio container renders with the correct style attribute.
#[doco::test]
async fn aspect_ratio_renders_with_style(client: Client) -> Result<()> {
    client.goto("/story/aspect-ratio").await?;
    tokio::time::sleep(std::time::Duration::from_millis(500)).await;

    let container = client
        .find(By::Css("main div[style*='aspect-ratio']"))
        .await?;
    let style = container.attr("style").await?.unwrap_or_default();
    assert!(
        style.contains("aspect-ratio"),
        "container should have an aspect-ratio style, got: {style}"
    );
    Ok(())
}

/// Widescreen variant applies a 16:9 aspect ratio.
#[doco::test]
async fn widescreen_variant_has_correct_ratio(client: Client) -> Result<()> {
    client.goto("/story/aspect-ratio").await?;
    tokio::time::sleep(std::time::Duration::from_millis(500)).await;

    let containers = client
        .find_all(By::Css("main div[style*='aspect-ratio']"))
        .await?;

    // The widescreen variant (16:9 ≈ 1.777...) is the second variant
    let widescreen = &containers[1];
    let style = widescreen.attr("style").await?.unwrap_or_default();
    assert!(
        style.contains("1.777"),
        "widescreen variant should have 16:9 ratio (~1.777), got: {style}"
    );
    Ok(())
}
