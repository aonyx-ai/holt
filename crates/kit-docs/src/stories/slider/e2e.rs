use doco::{Client, Result};
use thirtyfour::By;

/// Slider component renders with the correct ARIA role.
#[doco::test]
async fn slider_renders_with_role(client: Client) -> Result<()> {
    client.goto("/story/slider").await?;
    tokio::time::sleep(std::time::Duration::from_millis(500)).await;

    let slider = client.find(By::Css("main [role='slider']")).await?;
    assert!(slider.is_displayed().await?, "slider should be visible");
    Ok(())
}

/// Slider component has correct default ARIA attributes.
#[doco::test]
async fn slider_has_aria_attributes(client: Client) -> Result<()> {
    client.goto("/story/slider").await?;
    tokio::time::sleep(std::time::Duration::from_millis(500)).await;

    let slider = client.find(By::Css("main [role='slider']")).await?;
    let aria_min = slider.attr("aria-valuemin").await?;
    let aria_max = slider.attr("aria-valuemax").await?;
    assert_eq!(aria_min.as_deref(), Some("0"));
    assert_eq!(aria_max.as_deref(), Some("100"));
    Ok(())
}

/// Disabled slider has aria-disabled attribute.
#[doco::test]
async fn disabled_slider_has_aria_disabled(client: Client) -> Result<()> {
    client.goto("/story/slider/disabled").await?;
    tokio::time::sleep(std::time::Duration::from_millis(500)).await;

    let slider = client.find(By::Css("main [role='slider']")).await?;
    let aria_disabled = slider.attr("aria-disabled").await?;
    assert_eq!(
        aria_disabled.as_deref(),
        Some("true"),
        "disabled slider should have aria-disabled=true"
    );
    Ok(())
}
