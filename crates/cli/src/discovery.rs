//! Story discovery - scrapes the storybook to find all stories and variants

use std::time::Duration;
use thirtyfour::prelude::*;

/// Story variant metadata extracted from the storybook
#[derive(Debug, Clone)]
pub struct StoryVariant {
    pub story_id: String,
    pub variant_index: usize,
    pub name: String,
}

/// Discovers all stories and variants by scraping the storybook navigation
pub async fn discover_stories(
    driver: &WebDriver,
    base_url: &str,
) -> WebDriverResult<Vec<StoryVariant>> {
    println!("Discovering stories...");

    // Navigate to the storybook home page
    driver.goto(&format!("{}/", base_url)).await?;

    // Wait for stories to load
    tokio::time::sleep(Duration::from_millis(1000)).await;

    // Get all story links from the navigation
    let story_links = driver.find_all(By::Css("nav a[href^='/story/']")).await?;

    let mut story_ids = Vec::new();
    for link in story_links {
        if let Ok(href) = link.attr("href").await
            && let Some(href_str) = href
        {
            // Extract story ID from href like "/story/button"
            if let Some(id) = href_str.strip_prefix("/story/") {
                story_ids.push(id.to_string());
            }
        }
    }

    println!("Found {} stories", story_ids.len());

    // For each story, navigate to it and count variants
    let mut variants = Vec::new();
    for story_id in story_ids {
        driver
            .goto(&format!("{}/story/{}", base_url, story_id))
            .await?;
        tokio::time::sleep(Duration::from_millis(500)).await;

        // Find the select element for variants
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
