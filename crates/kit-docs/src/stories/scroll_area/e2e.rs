use doco::{Client, Result};
use thirtyfour::By;

/// Scroll area renders a scrollable container with content.
#[doco::test]
async fn scroll_area_renders_with_content(client: Client) -> Result<()> {
    client.goto("/story/scroll-area").await?;
    tokio::time::sleep(std::time::Duration::from_millis(500)).await;

    let scroll_area = client.find(By::Css("main div.overflow-y-auto")).await?;
    assert!(
        scroll_area.is_displayed().await?,
        "scroll area should be visible"
    );

    let tags = scroll_area.find_all(By::Css("div.text-sm")).await?;
    assert!(!tags.is_empty(), "scroll area should contain tag items");
    Ok(())
}

/// Scroll area horizontal variant renders with horizontal overflow.
#[doco::test]
async fn scroll_area_horizontal_variant(client: Client) -> Result<()> {
    client.goto("/story/scroll-area?variant=horizontal").await?;
    tokio::time::sleep(std::time::Duration::from_millis(500)).await;

    let scroll_area = client.find(By::Css("main div.overflow-x-auto")).await?;
    assert!(
        scroll_area.is_displayed().await?,
        "horizontal scroll area should be visible"
    );
    Ok(())
}
