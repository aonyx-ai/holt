use doco::{Client, Result};
use thirtyfour::By;

/// Avatar component renders as a <span> with rounded-full styling.
#[doco::test]
async fn avatar_renders_as_span(client: Client) -> Result<()> {
    client.goto("/story/avatar").await?;
    tokio::time::sleep(std::time::Duration::from_millis(500)).await;

    let avatar = client.find(By::Css("main span.rounded-full")).await?;
    let tag = avatar.tag_name().await?;
    assert_eq!(
        tag.to_lowercase(),
        "span",
        "Avatar root element should be a <span>"
    );
    Ok(())
}

/// Avatar fallback displays initials when no image is provided.
#[doco::test]
async fn avatar_fallback_displays_initials(client: Client) -> Result<()> {
    client.goto("/story/avatar").await?;
    tokio::time::sleep(std::time::Duration::from_millis(500)).await;

    let fallback = client
        .find(By::Css("main span.rounded-full span.bg-muted"))
        .await?;
    let text = fallback.text().await?;
    assert!(
        !text.is_empty(),
        "Fallback should display text content (initials)"
    );
    Ok(())
}

/// Avatar small variant applies correct size classes.
#[doco::test]
async fn avatar_small_has_correct_size(client: Client) -> Result<()> {
    client.goto("/story/avatar").await?;
    tokio::time::sleep(std::time::Duration::from_millis(500)).await;

    let small_avatar = client.find(By::Css("main span.h-8.w-8")).await?;
    assert!(
        small_avatar.is_displayed().await?,
        "Small avatar should be visible with h-8 w-8 classes"
    );
    Ok(())
}

/// Avatar large variant applies correct size classes.
#[doco::test]
async fn avatar_large_has_correct_size(client: Client) -> Result<()> {
    client.goto("/story/avatar").await?;
    tokio::time::sleep(std::time::Duration::from_millis(500)).await;

    let large_avatar = client.find(By::Css("main span.h-12.w-12")).await?;
    assert!(
        large_avatar.is_displayed().await?,
        "Large avatar should be visible with h-12 w-12 classes"
    );
    Ok(())
}
