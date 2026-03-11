use doco::{Client, Result};
use thirtyfour::By;

/// All registered stories should appear as sidebar navigation links.
#[doco::test]
async fn sidebar_lists_all_stories(client: Client) -> Result<()> {
    client.goto("/").await?;
    tokio::time::sleep(std::time::Duration::from_millis(500)).await;

    let nav_links = client.find_all(By::Css("nav a")).await?;
    let texts: Vec<String> = futures::future::join_all(nav_links.iter().map(|link| link.text()))
        .await
        .into_iter()
        .collect::<std::result::Result<Vec<_>, _>>()?;

    for expected in ["Badge", "Button", "Select", "Input", "Card"] {
        assert!(
            texts.iter().any(|t| t.contains(expected)),
            "missing {expected} nav link, found: {texts:?}",
        );
    }
    Ok(())
}

/// Navigating to a story page shows its heading and variant selector.
#[doco::test]
async fn story_page_renders_heading_and_variants(client: Client) -> Result<()> {
    client.goto("/story/button").await?;
    tokio::time::sleep(std::time::Duration::from_millis(500)).await;

    let heading = client.find(By::Css("main h1")).await?;
    assert_eq!(heading.text().await?, "Button");

    let options = client.find_all(By::Css("select option")).await?;
    assert!(
        options.len() >= 2,
        "expected at least 2 variants, found {}",
        options.len()
    );
    Ok(())
}
