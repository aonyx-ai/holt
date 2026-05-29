use doco::{Client, Result};
use thirtyfour::By;

/// Hover card trigger renders and responds to hover.
#[doco::test]
async fn hover_card_renders_trigger(client: Client) -> Result<()> {
    client.goto("/story/hover-card").await?;
    tokio::time::sleep(std::time::Duration::from_millis(500)).await;

    let trigger = client
        .find(By::Css("main [data-state] .inline-block"))
        .await?;
    assert!(
        trigger.is_displayed().await?,
        "trigger element should be visible"
    );
    Ok(())
}

/// Hover card content appears after hovering the trigger.
#[doco::test]
async fn hover_card_opens_on_hover(client: Client) -> Result<()> {
    client.goto("/story/hover-card").await?;
    tokio::time::sleep(std::time::Duration::from_millis(500)).await;

    let trigger = client
        .find(By::Css("main [data-state] .inline-block"))
        .await?;
    trigger.hover().await?;

    // Wait for the default open delay (700ms) plus a buffer
    tokio::time::sleep(std::time::Duration::from_millis(1000)).await;

    let content = client.find(By::Css("[data-state='open']")).await?;
    assert!(
        content.is_displayed().await?,
        "hover card content should be visible after hovering"
    );
    Ok(())
}

/// Hover card root starts in a closed state.
#[doco::test]
async fn hover_card_starts_closed(client: Client) -> Result<()> {
    client.goto("/story/hover-card").await?;
    tokio::time::sleep(std::time::Duration::from_millis(500)).await;

    let root = client.find(By::Css("main [data-state='closed']")).await?;
    let state = root.attr("data-state").await?;
    assert_eq!(
        state.as_deref(),
        Some("closed"),
        "hover card should start in closed state"
    );
    Ok(())
}
