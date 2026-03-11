use doco::{Client, Result};
use thirtyfour::By;

/// Badge component renders as a <span> element.
#[doco::test]
async fn badge_renders_as_span(client: Client) -> Result<()> {
    client.goto("/story/badge").await?;
    tokio::time::sleep(std::time::Duration::from_millis(500)).await;

    let badge = client.find(By::Css("main span.inline-flex")).await?;
    let tag = badge.tag_name().await?;
    assert_eq!(
        tag.to_lowercase(),
        "span",
        "Badge root element should be a <span>"
    );
    Ok(())
}
