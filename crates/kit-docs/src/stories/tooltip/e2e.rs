use doco::{Client, Result};
use thirtyfour::By;

/// Tooltip component shows content on hover with role="tooltip".
#[doco::test]
async fn tooltip_shows_on_hover(client: Client) -> Result<()> {
    client.goto("/story/tooltip").await?;
    tokio::time::sleep(std::time::Duration::from_millis(500)).await;

    // The "No delay" variant (custom_delay) uses open_delay_ms=0, so hovering
    // should reveal the tooltip immediately without waiting for a timer.
    let triggers: Vec<_> = client.find_all(By::Css("main span[data-state]")).await?;
    assert!(
        triggers.len() >= 2,
        "expected at least 2 tooltip triggers on the page"
    );

    // Hover over the instant-open trigger (second variant).
    let instant_trigger = &triggers[1];
    instant_trigger.scroll_into_view().await?;
    client
        .action_chain()
        .move_to_element(instant_trigger)
        .perform()
        .await?;
    tokio::time::sleep(std::time::Duration::from_millis(300)).await;

    let tooltip = client.find(By::Css("[role='tooltip']")).await?;
    assert!(
        tooltip.is_displayed().await?,
        "tooltip should be visible after hovering the trigger"
    );

    let text = tooltip.text().await?;
    assert_eq!(text, "Instant tooltip");
    Ok(())
}

/// Tooltip trigger has correct data-state attribute.
#[doco::test]
async fn tooltip_trigger_has_data_state(client: Client) -> Result<()> {
    client.goto("/story/tooltip").await?;
    tokio::time::sleep(std::time::Duration::from_millis(500)).await;

    let trigger = client.find(By::Css("main span[data-state]")).await?;
    let state = trigger.attr("data-state").await?;
    assert_eq!(
        state.as_deref(),
        Some("closed"),
        "trigger should start with data-state='closed'"
    );
    Ok(())
}
