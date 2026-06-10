use doco::{Client, Result};
use thirtyfour::By;

/// Alert component renders with the correct ARIA role.
#[doco::test]
async fn alert_renders_with_role(client: Client) -> Result<()> {
    client.goto("/story/alert").await?;
    tokio::time::sleep(std::time::Duration::from_millis(500)).await;

    let alert = client.find(By::Css("main [role='alert']")).await?;
    assert!(alert.is_displayed().await?);
    Ok(())
}

/// Alert title renders with the correct data-slot attribute.
#[doco::test]
async fn alert_title_has_data_slot(client: Client) -> Result<()> {
    client.goto("/story/alert").await?;
    tokio::time::sleep(std::time::Duration::from_millis(500)).await;

    let title = client
        .find(By::Css("main [data-slot='alert-title']"))
        .await?;
    assert!(title.is_displayed().await?);
    Ok(())
}

/// Alert description renders with the correct data-slot attribute.
#[doco::test]
async fn alert_description_has_data_slot(client: Client) -> Result<()> {
    client.goto("/story/alert").await?;
    tokio::time::sleep(std::time::Duration::from_millis(500)).await;

    let desc = client
        .find(By::Css("main [data-slot='alert-description']"))
        .await?;
    assert!(desc.is_displayed().await?);
    Ok(())
}
