//! Story discovery and screenshot capture.

use std::path::{Path, PathBuf};
use std::time::Duration;
use thirtyfour::prelude::*;

/// Story metadata extracted from the storybook.
#[derive(Debug, Clone)]
pub struct StoryVariant {
    pub story_id: String,
    pub variant_index: usize,
    pub name: String,
}

/// Discovers all stories and variants by scraping the storybook navigation.
pub async fn discover_stories(
    driver: &WebDriver,
    server_url: &str,
) -> WebDriverResult<Vec<StoryVariant>> {
    println!("Discovering stories...");

    driver.goto(&format!("{}/", server_url)).await?;

    // Wait for the nav to be rendered (WASM needs time to hydrate)
    let mut retries = 0;
    let story_links = loop {
        tokio::time::sleep(Duration::from_millis(1000)).await;
        let links = driver.find_all(By::Css("nav a[href^='/story/']")).await?;
        if !links.is_empty() || retries >= 10 {
            break links;
        }
        retries += 1;
        println!("Waiting for stories to load... ({}/10)", retries);
    };

    let mut story_ids = Vec::new();
    for link in story_links {
        if let Ok(href) = link.attr("href").await
            && let Some(href_str) = href
            && let Some(id) = href_str.strip_prefix("/story/")
        {
            story_ids.push(id.to_string());
        }
    }

    println!("Found {} stories", story_ids.len());

    let mut variants = Vec::new();
    for story_id in story_ids {
        driver
            .goto(&format!("{}/story/{}", server_url, story_id))
            .await?;
        tokio::time::sleep(Duration::from_millis(500)).await;

        if let Ok(select) = driver.find(By::Css("select")).await
            && let Ok(options) = select.find_all(By::Tag("option")).await
        {
            for (idx, option) in options.iter().enumerate() {
                if let Ok(name) = option.text().await {
                    variants.push(StoryVariant {
                        story_id: story_id.clone(),
                        variant_index: idx,
                        name: name.to_lowercase().replace(' ', "_"),
                    });
                }
            }
        }
    }

    println!("Found {} total story variants", variants.len());
    Ok(variants)
}

/// Captures a screenshot of a story variant.
pub async fn capture_screenshot(
    driver: &WebDriver,
    server_url: &str,
    variant: &StoryVariant,
) -> WebDriverResult<Vec<u8>> {
    let url = format!(
        "{}/visual-test/{}/{}",
        server_url, variant.story_id, variant.variant_index
    );

    println!("  Capturing: {}/{}", variant.story_id, variant.name);
    driver.goto(&url).await?;

    tokio::time::sleep(Duration::from_millis(500)).await;

    driver.screenshot_as_png().await
}

/// Gets the baseline path for a story variant.
pub fn get_baseline_path(baseline_dir: &Path, variant: &StoryVariant) -> PathBuf {
    baseline_dir
        .join(&variant.story_id)
        .join(format!("{}.png", variant.name))
}
