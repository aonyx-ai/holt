use doco::{Client, Result};
use thirtyfour::By;

/// Sheet trigger opens a dialog overlay on click.
#[doco::test]
async fn sheet_opens_on_trigger_click(client: Client) -> Result<()> {
    client.goto("/story/sheet").await?;
    tokio::time::sleep(std::time::Duration::from_millis(500)).await;

    let trigger = client.find(By::Css("main button")).await?;
    trigger.click().await?;
    tokio::time::sleep(std::time::Duration::from_millis(300)).await;

    let dialog = client.find(By::Css("[role='dialog']")).await?;
    assert!(
        dialog.is_displayed().await?,
        "sheet dialog should be visible"
    );
    Ok(())
}

/// Sheet overlay contains a close button.
#[doco::test]
async fn sheet_has_close_button(client: Client) -> Result<()> {
    client.goto("/story/sheet").await?;
    tokio::time::sleep(std::time::Duration::from_millis(500)).await;

    let trigger = client.find(By::Css("main button")).await?;
    trigger.click().await?;
    tokio::time::sleep(std::time::Duration::from_millis(300)).await;

    let close_btn = client
        .find(By::Css("[role='dialog'] button .sr-only"))
        .await?;
    let text = close_btn.text().await?;
    assert_eq!(
        text, "Close",
        "close button should have sr-only text 'Close'"
    );
    Ok(())
}

/// Sheet closes when the close button is clicked.
#[doco::test]
async fn sheet_closes_on_close_button(client: Client) -> Result<()> {
    client.goto("/story/sheet").await?;
    tokio::time::sleep(std::time::Duration::from_millis(500)).await;

    let trigger = client.find(By::Css("main button")).await?;
    trigger.click().await?;
    tokio::time::sleep(std::time::Duration::from_millis(300)).await;

    let dialog = client.find(By::Css("[role='dialog']")).await?;
    assert!(dialog.is_displayed().await?, "sheet should be open");

    let close_btn = client.find(By::Css("[role='dialog'] button")).await?;
    close_btn.click().await?;
    tokio::time::sleep(std::time::Duration::from_millis(300)).await;

    let dialogs = client.find_all(By::Css("[role='dialog']")).await?;
    assert!(
        dialogs.is_empty(),
        "sheet should be closed after clicking close"
    );
    Ok(())
}
