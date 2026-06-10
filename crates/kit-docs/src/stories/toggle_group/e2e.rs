use doco::{Client, Result};
use thirtyfour::By;

/// Toggle group renders a group container with toggle buttons.
#[doco::test]
async fn toggle_group_renders_group_with_items(client: Client) -> Result<()> {
    client.goto("/story/toggle-group").await?;
    tokio::time::sleep(std::time::Duration::from_millis(500)).await;

    let group = client.find(By::Css("main [role='group']")).await?;
    assert!(
        group.is_displayed().await?,
        "toggle group should be visible"
    );

    let items = group.find_all(By::Css("button[role='button']")).await?;
    assert_eq!(items.len(), 3, "toggle group should contain three items");
    Ok(())
}

/// Clicking a toggle group item in single mode activates it.
#[doco::test]
async fn single_mode_activates_on_click(client: Client) -> Result<()> {
    client.goto("/story/toggle-group").await?;
    tokio::time::sleep(std::time::Duration::from_millis(500)).await;

    let group = client.find(By::Css("main [role='group']")).await?;
    let items = group.find_all(By::Css("button[role='button']")).await?;

    items[0].click().await?;
    tokio::time::sleep(std::time::Duration::from_millis(200)).await;

    let state = items[0].attr("data-state").await?;
    assert_eq!(
        state.as_deref(),
        Some("on"),
        "clicked item should have data-state=on"
    );
    Ok(())
}

/// In single mode, clicking a second item deselects the first.
#[doco::test]
async fn single_mode_deselects_previous(client: Client) -> Result<()> {
    client.goto("/story/toggle-group").await?;
    tokio::time::sleep(std::time::Duration::from_millis(500)).await;

    let group = client.find(By::Css("main [role='group']")).await?;
    let items = group.find_all(By::Css("button[role='button']")).await?;

    items[0].click().await?;
    tokio::time::sleep(std::time::Duration::from_millis(200)).await;

    items[1].click().await?;
    tokio::time::sleep(std::time::Duration::from_millis(200)).await;

    let first_state = items[0].attr("data-state").await?;
    let second_state = items[1].attr("data-state").await?;
    assert_eq!(
        first_state.as_deref(),
        Some("off"),
        "first item should be deselected"
    );
    assert_eq!(
        second_state.as_deref(),
        Some("on"),
        "second item should be selected"
    );
    Ok(())
}

/// Disabled toggle group items cannot be clicked.
#[doco::test]
async fn disabled_group_items_are_not_clickable(client: Client) -> Result<()> {
    client.goto("/story/toggle-group?variant=disabled").await?;
    tokio::time::sleep(std::time::Duration::from_millis(500)).await;

    let group = client.find(By::Css("main [role='group']")).await?;
    let items = group.find_all(By::Css("button[role='button']")).await?;

    let is_disabled = items[0].prop("disabled").await?;
    assert_eq!(
        is_disabled.as_deref(),
        Some("true"),
        "disabled group items should have disabled property"
    );
    Ok(())
}
