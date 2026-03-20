use doco::{Client, Result};
use thirtyfour::By;

/// Menubar renders trigger buttons with correct ARIA roles.
#[doco::test]
async fn menubar_renders_triggers(client: Client) -> Result<()> {
    client.goto("/story/menubar").await?;
    tokio::time::sleep(std::time::Duration::from_millis(500)).await;

    let menubar = client.find(By::Css("main [role='menubar']")).await?;
    assert!(menubar.is_displayed().await?, "menubar should be visible");

    let triggers = menubar.find_all(By::Css("button[role='menuitem']")).await?;
    assert_eq!(triggers.len(), 3, "should have three menu triggers");
    Ok(())
}

/// Clicking a trigger opens its menu content.
#[doco::test]
async fn menubar_opens_on_click(client: Client) -> Result<()> {
    client.goto("/story/menubar").await?;
    tokio::time::sleep(std::time::Duration::from_millis(500)).await;

    let trigger = client
        .find(By::Css("main [role='menubar'] button[role='menuitem']"))
        .await?;
    trigger.click().await?;
    tokio::time::sleep(std::time::Duration::from_millis(300)).await;

    let menu = client.find(By::Css("[role='menu']")).await?;
    assert!(menu.is_displayed().await?, "menu should be visible");

    let items = menu.find_all(By::Css("[role='menuitem']")).await?;
    assert!(!items.is_empty(), "menu should contain at least one item");
    Ok(())
}

/// Clicking a trigger sets aria-expanded to true.
#[doco::test]
async fn menubar_trigger_aria_expanded(client: Client) -> Result<()> {
    client.goto("/story/menubar").await?;
    tokio::time::sleep(std::time::Duration::from_millis(500)).await;

    let trigger = client
        .find(By::Css("main [role='menubar'] button[role='menuitem']"))
        .await?;

    let before = trigger.attr("aria-expanded").await?;
    assert_eq!(before.as_deref(), Some("false"));

    trigger.click().await?;
    tokio::time::sleep(std::time::Duration::from_millis(300)).await;

    let after = trigger.attr("aria-expanded").await?;
    assert_eq!(after.as_deref(), Some("true"));
    Ok(())
}
