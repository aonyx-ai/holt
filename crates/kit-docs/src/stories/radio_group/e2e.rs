use doco::{Client, Result};
use thirtyfour::By;

/// Radio group renders with correct ARIA role.
#[doco::test]
async fn radio_group_renders_radiogroup_role(client: Client) -> Result<()> {
    client.goto("/story/radio-group").await?;
    tokio::time::sleep(std::time::Duration::from_millis(500)).await;

    let group = client.find(By::Css("main [role='radiogroup']")).await?;
    assert!(
        group.is_displayed().await?,
        "radiogroup container should be visible"
    );
    Ok(())
}

/// Clicking a radio item selects it.
#[doco::test]
async fn radio_item_selects_on_click(client: Client) -> Result<()> {
    client.goto("/story/radio-group").await?;
    tokio::time::sleep(std::time::Duration::from_millis(500)).await;

    let items = client
        .find_all(By::Css("main [role='radiogroup'] button[role='radio']"))
        .await?;
    assert!(items.len() >= 2, "should have at least two radio items");

    // Click the first item
    items[0].click().await?;
    tokio::time::sleep(std::time::Duration::from_millis(300)).await;

    let state = items[0].attr("data-state").await?;
    assert_eq!(
        state.as_deref(),
        Some("checked"),
        "clicked radio item should be checked"
    );

    // Click the second item — first should become unchecked
    items[1].click().await?;
    tokio::time::sleep(std::time::Duration::from_millis(300)).await;

    let first_state = items[0].attr("data-state").await?;
    let second_state = items[1].attr("data-state").await?;
    assert_eq!(
        first_state.as_deref(),
        Some("unchecked"),
        "first item should be unchecked after selecting second"
    );
    assert_eq!(
        second_state.as_deref(),
        Some("checked"),
        "second item should be checked"
    );
    Ok(())
}

/// Radio items have correct aria-checked attributes.
#[doco::test]
async fn radio_items_have_aria_checked(client: Client) -> Result<()> {
    client.goto("/story/radio-group").await?;
    tokio::time::sleep(std::time::Duration::from_millis(500)).await;

    let items = client
        .find_all(By::Css("main [role='radiogroup'] button[role='radio']"))
        .await?;
    assert!(!items.is_empty(), "should find radio items");

    // Initially none should be checked in the default variant
    for item in &items {
        let checked = item.attr("aria-checked").await?;
        assert_eq!(
            checked.as_deref(),
            Some("false"),
            "radio items should start unchecked in default variant"
        );
    }

    // Click one and verify aria-checked updates
    items[0].click().await?;
    tokio::time::sleep(std::time::Duration::from_millis(300)).await;

    let checked = items[0].attr("aria-checked").await?;
    assert_eq!(
        checked.as_deref(),
        Some("true"),
        "clicked item should have aria-checked=true"
    );
    Ok(())
}
