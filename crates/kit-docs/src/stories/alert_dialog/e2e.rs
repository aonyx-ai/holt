use doco::{Client, Result};
use thirtyfour::By;

/// Alert dialog opens and displays overlay and content when trigger is clicked.
#[doco::test]
async fn alert_dialog_opens_on_trigger_click(client: Client) -> Result<()> {
    client.goto("/story/alert-dialog").await?;
    tokio::time::sleep(std::time::Duration::from_millis(500)).await;

    let trigger = client.find(By::Css("main button")).await?;
    trigger.click().await?;
    tokio::time::sleep(std::time::Duration::from_millis(300)).await;

    let overlay = client.find(By::Css("[data-state='open']")).await?;
    assert!(overlay.is_displayed().await?, "overlay should be visible");

    let dialog = client.find(By::Css("[role='alertdialog']")).await?;
    assert!(
        dialog.is_displayed().await?,
        "alert dialog content should be visible"
    );

    Ok(())
}

/// Clicking cancel closes the alert dialog.
#[doco::test]
async fn alert_dialog_cancel_closes(client: Client) -> Result<()> {
    client.goto("/story/alert-dialog").await?;
    tokio::time::sleep(std::time::Duration::from_millis(500)).await;

    let trigger = client.find(By::Css("main button")).await?;
    trigger.click().await?;
    tokio::time::sleep(std::time::Duration::from_millis(300)).await;

    let dialog = client.find(By::Css("[role='alertdialog']")).await?;
    assert!(
        dialog.is_displayed().await?,
        "dialog should be visible after opening"
    );

    let cancel = dialog
        .find(By::XPath(".//button[contains(text(),'Cancel')]"))
        .await?;
    cancel.click().await?;
    tokio::time::sleep(std::time::Duration::from_millis(300)).await;

    let dialogs = client.find_all(By::Css("[role='alertdialog']")).await?;
    assert!(
        dialogs.is_empty(),
        "dialog should be removed from DOM after cancel"
    );

    Ok(())
}

/// Alert dialog content includes title and description text.
#[doco::test]
async fn alert_dialog_shows_title_and_description(client: Client) -> Result<()> {
    client.goto("/story/alert-dialog").await?;
    tokio::time::sleep(std::time::Duration::from_millis(500)).await;

    let trigger = client.find(By::Css("main button")).await?;
    trigger.click().await?;
    tokio::time::sleep(std::time::Duration::from_millis(300)).await;

    let dialog = client.find(By::Css("[role='alertdialog']")).await?;

    let title = dialog.find(By::Css("h2")).await?;
    assert_eq!(
        title.text().await?,
        "Are you absolutely sure?",
        "dialog should display title"
    );

    let description = dialog.find(By::Css("p")).await?;
    let desc_text = description.text().await?;
    assert!(
        desc_text.contains("permanently delete"),
        "dialog should display description"
    );

    Ok(())
}
