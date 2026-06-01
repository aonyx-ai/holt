use doco::{Client, Result};
use thirtyfour::By;

/// Context menu trigger area renders with the expected content.
#[doco::test]
async fn context_menu_trigger_renders(client: Client) -> Result<()> {
    client.goto("/story/context-menu").await?;
    tokio::time::sleep(std::time::Duration::from_millis(500)).await;

    let trigger = client.find(By::Css("main div.border-dashed")).await?;
    let text = trigger.text().await?;
    assert!(
        text.contains("Right click here"),
        "trigger should display instructional text"
    );
    Ok(())
}

/// Context menu opens on right-click and shows menu items.
#[doco::test]
async fn context_menu_opens_on_right_click(client: Client) -> Result<()> {
    client.goto("/story/context-menu").await?;
    tokio::time::sleep(std::time::Duration::from_millis(500)).await;

    // Simulate a right-click via JavaScript contextmenu event dispatch
    let trigger = client.find(By::Css("main div.border-dashed")).await?;
    client
        .execute(
            "arguments[0].dispatchEvent(new MouseEvent('contextmenu', { bubbles: true, clientX: 150, clientY: 75 }))",
            vec![trigger.to_json()?],
        )
        .await?;
    tokio::time::sleep(std::time::Duration::from_millis(300)).await;

    let menu = client.find(By::Css("[role='menu']")).await?;
    assert!(menu.is_displayed().await?, "context menu should be visible");

    let items = menu.find_all(By::Css("[role='menuitem']")).await?;
    assert!(
        !items.is_empty(),
        "context menu should contain at least one item"
    );
    Ok(())
}

/// Context menu contains a separator between item groups.
#[doco::test]
async fn context_menu_has_separator(client: Client) -> Result<()> {
    client.goto("/story/context-menu").await?;
    tokio::time::sleep(std::time::Duration::from_millis(500)).await;

    // Open the menu
    let trigger = client.find(By::Css("main div.border-dashed")).await?;
    client
        .execute(
            "arguments[0].dispatchEvent(new MouseEvent('contextmenu', { bubbles: true, clientX: 150, clientY: 75 }))",
            vec![trigger.to_json()?],
        )
        .await?;
    tokio::time::sleep(std::time::Duration::from_millis(300)).await;

    let separators = client.find_all(By::Css("[role='separator']")).await?;
    assert!(
        !separators.is_empty(),
        "context menu should contain at least one separator"
    );
    Ok(())
}
