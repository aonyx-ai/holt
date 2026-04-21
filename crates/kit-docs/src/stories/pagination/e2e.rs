use doco::{Client, Result};
use thirtyfour::By;

/// Pagination component renders as a <nav> element with correct aria label.
#[doco::test]
async fn pagination_renders_as_nav(client: Client) -> Result<()> {
    client.goto("/story/pagination").await?;
    tokio::time::sleep(std::time::Duration::from_millis(500)).await;

    let nav = client
        .find(By::Css("main nav[aria-label='pagination']"))
        .await?;
    let tag = nav.tag_name().await?;
    assert_eq!(
        tag.to_lowercase(),
        "nav",
        "Pagination root element should be a <nav>"
    );
    Ok(())
}

/// The active page link has aria-current="page".
#[doco::test]
async fn active_link_has_aria_current(client: Client) -> Result<()> {
    client.goto("/story/pagination").await?;
    tokio::time::sleep(std::time::Duration::from_millis(500)).await;

    let active = client
        .find(By::Css("main nav a[aria-current='page']"))
        .await?;
    let text = active.text().await?;
    assert_eq!(text, "1", "Active page should be page 1 in default variant");
    Ok(())
}

/// Previous button has the correct aria label.
#[doco::test]
async fn previous_button_has_aria_label(client: Client) -> Result<()> {
    client.goto("/story/pagination").await?;
    tokio::time::sleep(std::time::Duration::from_millis(500)).await;

    let prev = client
        .find(By::Css("main nav a[aria-label='Go to previous page']"))
        .await?;
    let text = prev.text().await?;
    assert!(
        text.contains("Previous"),
        "Previous button should contain 'Previous' text"
    );
    Ok(())
}

/// Next button has the correct aria label.
#[doco::test]
async fn next_button_has_aria_label(client: Client) -> Result<()> {
    client.goto("/story/pagination").await?;
    tokio::time::sleep(std::time::Duration::from_millis(500)).await;

    let next = client
        .find(By::Css("main nav a[aria-label='Go to next page']"))
        .await?;
    let text = next.text().await?;
    assert!(
        text.contains("Next"),
        "Next button should contain 'Next' text"
    );
    Ok(())
}

/// Ellipsis elements are hidden from assistive technology.
#[doco::test]
async fn ellipsis_is_aria_hidden(client: Client) -> Result<()> {
    client.goto("/story/pagination/with-ellipsis").await?;
    tokio::time::sleep(std::time::Duration::from_millis(500)).await;

    let ellipsis = client
        .find(By::Css("main nav span[aria-hidden='true']"))
        .await?;
    let hidden = ellipsis.attr("aria-hidden").await?;
    assert_eq!(
        hidden.as_deref(),
        Some("true"),
        "Ellipsis should have aria-hidden='true'"
    );
    Ok(())
}
