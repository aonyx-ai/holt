use doco::{Client, Result};
use thirtyfour::By;

/// Popover trigger renders as a button with correct ARIA attributes.
#[doco::test]
async fn popover_trigger_has_aria_attributes(client: Client) -> Result<()> {
    client.goto("/story/popover").await?;
    tokio::time::sleep(std::time::Duration::from_millis(500)).await;

    let trigger = client
        .find(By::Css("main button[aria-haspopup='dialog']"))
        .await?;
    let expanded = trigger.attr("aria-expanded").await?;
    assert_eq!(
        expanded.as_deref(),
        Some("false"),
        "trigger should have aria-expanded=false when closed"
    );
    Ok(())
}

/// Clicking the trigger opens the popover dialog.
#[doco::test]
async fn popover_opens_on_trigger_click(client: Client) -> Result<()> {
    client.goto("/story/popover").await?;
    tokio::time::sleep(std::time::Duration::from_millis(500)).await;

    let trigger = client
        .find(By::Css("main button[aria-haspopup='dialog']"))
        .await?;
    trigger.click().await?;
    tokio::time::sleep(std::time::Duration::from_millis(300)).await;

    let dialog = client.find(By::Css("[role='dialog']")).await?;
    assert!(dialog.is_displayed().await?, "popover should be visible");

    let expanded = trigger.attr("aria-expanded").await?;
    assert_eq!(
        expanded.as_deref(),
        Some("true"),
        "trigger should have aria-expanded=true when open"
    );
    Ok(())
}

/// Clicking the trigger again closes the popover.
#[doco::test]
async fn popover_closes_on_second_click(client: Client) -> Result<()> {
    client.goto("/story/popover").await?;
    tokio::time::sleep(std::time::Duration::from_millis(500)).await;

    let trigger = client
        .find(By::Css("main button[aria-haspopup='dialog']"))
        .await?;
    trigger.click().await?;
    tokio::time::sleep(std::time::Duration::from_millis(300)).await;

    let dialog = client.find(By::Css("[role='dialog']")).await?;
    assert!(dialog.is_displayed().await?, "popover should open");

    trigger.click().await?;
    tokio::time::sleep(std::time::Duration::from_millis(300)).await;

    let dialogs = client.find_all(By::Css("[role='dialog']")).await?;
    assert!(
        dialogs.is_empty(),
        "popover should be removed from the DOM after closing"
    );
    Ok(())
}
