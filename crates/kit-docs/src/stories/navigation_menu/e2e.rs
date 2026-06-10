use doco::{Client, Result};
use thirtyfour::By;

/// Navigation menu renders a nav element with a menubar.
#[doco::test]
async fn navigation_menu_renders_nav_with_menubar(client: Client) -> Result<()> {
    client.goto("/story/navigation-menu").await?;
    tokio::time::sleep(std::time::Duration::from_millis(500)).await;

    let nav = client
        .find(By::Css("main [data-component='navigation-menu']"))
        .await?;
    let tag = nav.tag_name().await?;
    assert_eq!(
        tag.to_lowercase(),
        "nav",
        "NavigationMenu root should be a <nav> element"
    );

    let menubar = nav.find(By::Css("[role='menubar']")).await?;
    assert!(
        menubar.is_displayed().await?,
        "menubar list should be visible"
    );
    Ok(())
}

/// Clicking a trigger opens the associated content panel.
#[doco::test]
async fn trigger_opens_content_on_click(client: Client) -> Result<()> {
    client.goto("/story/navigation-menu").await?;
    tokio::time::sleep(std::time::Duration::from_millis(500)).await;

    let trigger = client.find(By::Css("main button[role='menuitem']")).await?;
    assert_eq!(
        trigger.attr("aria-expanded").await?.as_deref(),
        Some("false"),
        "trigger should start collapsed"
    );

    trigger.click().await?;
    tokio::time::sleep(std::time::Duration::from_millis(300)).await;

    assert_eq!(
        trigger.attr("aria-expanded").await?.as_deref(),
        Some("true"),
        "trigger should be expanded after click"
    );

    let content = client
        .find(By::Css("main [role='menu']:not([hidden])"))
        .await?;
    assert!(
        content.is_displayed().await?,
        "content panel should be visible after click"
    );
    Ok(())
}

/// Clicking a trigger twice closes the content panel.
#[doco::test]
async fn trigger_toggles_content_closed(client: Client) -> Result<()> {
    client.goto("/story/navigation-menu").await?;
    tokio::time::sleep(std::time::Duration::from_millis(500)).await;

    let trigger = client.find(By::Css("main button[role='menuitem']")).await?;

    trigger.click().await?;
    tokio::time::sleep(std::time::Duration::from_millis(300)).await;

    trigger.click().await?;
    tokio::time::sleep(std::time::Duration::from_millis(300)).await;

    assert_eq!(
        trigger.attr("aria-expanded").await?.as_deref(),
        Some("false"),
        "trigger should be collapsed after second click"
    );
    Ok(())
}

/// Content panel contains navigation links with correct roles.
#[doco::test]
async fn content_contains_menu_links(client: Client) -> Result<()> {
    client.goto("/story/navigation-menu").await?;
    tokio::time::sleep(std::time::Duration::from_millis(500)).await;

    let trigger = client.find(By::Css("main button[role='menuitem']")).await?;
    trigger.click().await?;
    tokio::time::sleep(std::time::Duration::from_millis(300)).await;

    let links = client
        .find_all(By::Css("main [role='menu'] a[role='menuitem']"))
        .await?;
    assert!(
        links.len() >= 2,
        "content panel should contain at least two navigation links"
    );
    Ok(())
}
