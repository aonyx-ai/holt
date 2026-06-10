use doco::{Client, Result};
use thirtyfour::By;

/// Dialog opens when the trigger button is clicked.
#[doco::test]
async fn dialog_opens_on_trigger_click(client: Client) -> Result<()> {
    client.goto("/story/dialog").await?;
    tokio::time::sleep(std::time::Duration::from_millis(500)).await;

    let trigger = client
        .find(By::Css("main button[aria-haspopup='dialog']"))
        .await?;
    trigger.click().await?;
    tokio::time::sleep(std::time::Duration::from_millis(300)).await;

    let dialog = client.find(By::Css("[role='dialog']")).await?;
    assert!(dialog.is_displayed().await?, "dialog should be visible");
    Ok(())
}

/// Dialog displays an overlay backdrop when open.
#[doco::test]
async fn dialog_displays_overlay(client: Client) -> Result<()> {
    client.goto("/story/dialog").await?;
    tokio::time::sleep(std::time::Duration::from_millis(500)).await;

    let trigger = client
        .find(By::Css("main button[aria-haspopup='dialog']"))
        .await?;
    trigger.click().await?;
    tokio::time::sleep(std::time::Duration::from_millis(300)).await;

    let backdrop = client.find(By::Css(".dialog-backdrop")).await?;
    assert!(
        backdrop.is_displayed().await?,
        "backdrop overlay should be visible"
    );
    Ok(())
}

/// Dialog closes when the close button is clicked.
#[doco::test]
async fn dialog_closes_on_close_button(client: Client) -> Result<()> {
    client.goto("/story/dialog").await?;
    tokio::time::sleep(std::time::Duration::from_millis(500)).await;

    let trigger = client
        .find(By::Css("main button[aria-haspopup='dialog']"))
        .await?;
    trigger.click().await?;
    tokio::time::sleep(std::time::Duration::from_millis(300)).await;

    let dialog = client.find(By::Css("[role='dialog']")).await?;
    assert!(dialog.is_displayed().await?, "dialog should be visible");

    // Click the X close button (the one with sr-only "Close" text)
    let close_btn = client
        .find(By::Css("[role='dialog'] button:has(.sr-only)"))
        .await?;
    close_btn.click().await?;
    tokio::time::sleep(std::time::Duration::from_millis(300)).await;

    let dialogs = client.find_all(By::Css("[role='dialog']")).await?;
    assert!(dialogs.is_empty(), "dialog should be closed");
    Ok(())
}
