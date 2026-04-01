use doco::{Client, Result};
use thirtyfour::By;

/// Carousel renders with correct ARIA attributes.
#[doco::test]
async fn carousel_renders(client: Client) -> Result<()> {
    client.goto("/story/carousel").await?;
    tokio::time::sleep(std::time::Duration::from_millis(500)).await;

    let carousel = client
        .find(By::Css("main [aria-roledescription='carousel']"))
        .await?;
    assert!(
        carousel.is_displayed().await?,
        "carousel region should be visible"
    );

    let slide_group = client
        .find(By::Css("main [aria-roledescription='slide-group']"))
        .await?;
    assert!(
        slide_group.is_displayed().await?,
        "slide group should be visible"
    );

    let slides = client
        .find_all(By::Css("main [aria-roledescription='slide']"))
        .await?;
    assert!(
        slides.len() >= 3,
        "carousel should contain at least 3 slides"
    );
    Ok(())
}

/// Previous and Next navigation buttons are present and clickable.
#[doco::test]
async fn navigation_buttons_work(client: Client) -> Result<()> {
    client.goto("/story/carousel").await?;
    tokio::time::sleep(std::time::Duration::from_millis(500)).await;

    let prev = client
        .find(By::Css("main button[aria-label='Previous slide']"))
        .await?;
    assert!(
        prev.is_displayed().await?,
        "previous button should be visible"
    );

    let next = client
        .find(By::Css("main button[aria-label='Next slide']"))
        .await?;
    assert!(next.is_displayed().await?, "next button should be visible");

    // Click next, then prev — no errors means the scroll handlers fired.
    next.click().await?;
    tokio::time::sleep(std::time::Duration::from_millis(400)).await;
    prev.click().await?;
    tokio::time::sleep(std::time::Duration::from_millis(400)).await;

    Ok(())
}
