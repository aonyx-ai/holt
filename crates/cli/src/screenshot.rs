//! Screenshot capture functionality

use crate::discovery::StoryVariant;
use std::time::Duration;
use thirtyfour::prelude::*;

/// Captures a screenshot of a story variant
pub async fn capture_screenshot(
    driver: &WebDriver,
    base_url: &str,
    variant: &StoryVariant,
) -> WebDriverResult<Vec<u8>> {
    let url = format!(
        "{}/visual-test/{}/{}",
        base_url, variant.story_id, variant.variant_index
    );

    driver.goto(&url).await?;

    // Wait a bit for rendering to complete
    tokio::time::sleep(Duration::from_millis(500)).await;

    driver.screenshot_as_png().await
}
