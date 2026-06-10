use doco::{Client, Result};
use thirtyfour::By;

/// Accordion renders and items can be toggled.
#[doco::test]
async fn accordion_item_toggles(client: Client) -> Result<()> {
    client.goto("/story/accordion").await?;
    tokio::time::sleep(std::time::Duration::from_millis(500)).await;

    // Find all trigger buttons inside the accordion
    let triggers = client
        .find_all(By::Css("main button[aria-expanded]"))
        .await?;
    assert!(
        triggers.len() >= 2,
        "should have at least two accordion triggers"
    );

    // All items should start closed
    let first = &triggers[0];
    assert_eq!(
        first.attr("aria-expanded").await?.as_deref(),
        Some("false"),
        "first item should start collapsed"
    );

    // Click the first trigger to open it
    first.click().await?;
    tokio::time::sleep(std::time::Duration::from_millis(300)).await;

    assert_eq!(
        first.attr("aria-expanded").await?.as_deref(),
        Some("true"),
        "first item should be expanded after click"
    );

    // In single mode, clicking a second trigger should close the first
    let second = &triggers[1];
    second.click().await?;
    tokio::time::sleep(std::time::Duration::from_millis(300)).await;

    assert_eq!(
        second.attr("aria-expanded").await?.as_deref(),
        Some("true"),
        "second item should be expanded after click"
    );
    assert_eq!(
        first.attr("aria-expanded").await?.as_deref(),
        Some("false"),
        "first item should collapse when another opens in single mode"
    );

    Ok(())
}
