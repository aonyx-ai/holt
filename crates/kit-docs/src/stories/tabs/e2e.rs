use doco::{Client, Result};
use thirtyfour::By;

/// Tabs component renders with the default tab active.
#[doco::test]
async fn tabs_renders_default_active(client: Client) -> Result<()> {
    client.goto("/story/tabs").await?;
    tokio::time::sleep(std::time::Duration::from_millis(500)).await;

    let active_trigger = client
        .find(By::Css("main button[role='tab'][data-state='active']"))
        .await?;
    let text = active_trigger.text().await?;
    assert_eq!(text, "Account", "default tab should be Account");

    let active_panel = client
        .find(By::Css("main div[role='tabpanel']:not([hidden])"))
        .await?;
    let panel_text = active_panel.text().await?;
    assert!(
        panel_text.contains("Account"),
        "active panel should show Account content"
    );

    Ok(())
}

/// Clicking a tab trigger switches the visible content panel.
#[doco::test]
async fn tabs_switch_content_on_click(client: Client) -> Result<()> {
    client.goto("/story/tabs").await?;
    tokio::time::sleep(std::time::Duration::from_millis(500)).await;

    // Click the Password tab
    let triggers = client
        .find_all(By::Css("main button[role='tab']"))
        .await?;
    assert!(triggers.len() >= 2, "should have at least two tab triggers");

    triggers[1].click().await?;
    tokio::time::sleep(std::time::Duration::from_millis(300)).await;

    // Verify the Password panel is now visible
    let active_panel = client
        .find(By::Css("main div[role='tabpanel']:not([hidden])"))
        .await?;
    let panel_text = active_panel.text().await?;
    assert!(
        panel_text.contains("Password"),
        "panel should show Password content after clicking Password tab"
    );

    // Verify the Password trigger is now active
    let password_trigger = &triggers[1];
    let state = password_trigger.attr("data-state").await?;
    assert_eq!(
        state.as_deref(),
        Some("active"),
        "Password trigger should be active"
    );

    Ok(())
}

/// A disabled tab trigger cannot be activated.
#[doco::test]
async fn tabs_disabled_trigger_not_activatable(client: Client) -> Result<()> {
    client.goto("/story/tabs").await?;
    tokio::time::sleep(std::time::Duration::from_millis(500)).await;

    // Find the with_disabled variant's disabled trigger
    let disabled_triggers = client
        .find_all(By::Css("main button[role='tab'][disabled]"))
        .await?;
    assert!(
        !disabled_triggers.is_empty(),
        "should have at least one disabled trigger"
    );

    let disabled = &disabled_triggers[0];
    let state_before = disabled.attr("data-state").await?;
    assert_eq!(
        state_before.as_deref(),
        Some("inactive"),
        "disabled trigger should be inactive"
    );

    disabled.click().await?;
    tokio::time::sleep(std::time::Duration::from_millis(300)).await;

    let state_after = disabled.attr("data-state").await?;
    assert_eq!(
        state_after.as_deref(),
        Some("inactive"),
        "disabled trigger should remain inactive after click"
    );

    Ok(())
}
