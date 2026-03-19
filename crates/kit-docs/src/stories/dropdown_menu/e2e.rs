use doco::{Client, Result};
use thirtyfour::By;

/// Dropdown menu trigger renders as a button with correct ARIA attributes.
#[doco::test]
async fn dropdown_menu_trigger_has_aria_attributes(client: Client) -> Result<()> {
    client.goto("/story/dropdown-menu").await?;
    tokio::time::sleep(std::time::Duration::from_millis(500)).await;

    let trigger = client
        .find(By::Css("main button[aria-haspopup='menu']"))
        .await?;
    let expanded = trigger.attr("aria-expanded").await?;
    assert_eq!(
        expanded.as_deref(),
        Some("false"),
        "trigger should have aria-expanded=false when closed"
    );
    Ok(())
}

/// Clicking the trigger opens the dropdown menu.
#[doco::test]
async fn dropdown_menu_opens_on_click(client: Client) -> Result<()> {
    client.goto("/story/dropdown-menu").await?;
    tokio::time::sleep(std::time::Duration::from_millis(500)).await;

    let trigger = client
        .find(By::Css("main button[aria-haspopup='menu']"))
        .await?;
    trigger.click().await?;
    tokio::time::sleep(std::time::Duration::from_millis(300)).await;

    let menu = client.find(By::Css("[role='menu']")).await?;
    assert!(menu.is_displayed().await?, "menu should be visible");

    let items = menu.find_all(By::Css("[role='menuitem']")).await?;
    assert!(
        !items.is_empty(),
        "menu should contain at least one menu item"
    );
    Ok(())
}

/// Menu items are rendered with the correct role.
#[doco::test]
async fn dropdown_menu_items_have_menuitem_role(client: Client) -> Result<()> {
    client.goto("/story/dropdown-menu").await?;
    tokio::time::sleep(std::time::Duration::from_millis(500)).await;

    let trigger = client
        .find(By::Css("main button[aria-haspopup='menu']"))
        .await?;
    trigger.click().await?;
    tokio::time::sleep(std::time::Duration::from_millis(300)).await;

    let items = client.find_all(By::Css("[role='menu'] [role='menuitem']")).await?;
    assert!(
        items.len() >= 4,
        "default variant should have at least 4 menu items, found {}",
        items.len()
    );
    Ok(())
}
