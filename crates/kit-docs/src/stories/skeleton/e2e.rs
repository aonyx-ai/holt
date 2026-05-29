use doco::{Client, Result};
use thirtyfour::By;

/// Skeleton component renders as a <div> with the animate-pulse class.
#[doco::test]
async fn skeleton_renders_with_pulse_animation(client: Client) -> Result<()> {
    client.goto("/story/skeleton").await?;
    tokio::time::sleep(std::time::Duration::from_millis(500)).await;

    let skeleton = client.find(By::Css("main div.animate-pulse")).await?;
    let tag = skeleton.tag_name().await?;
    assert_eq!(
        tag.to_lowercase(),
        "div",
        "Skeleton root element should be a <div>"
    );
    Ok(())
}

/// Skeleton card variant renders multiple skeleton elements.
#[doco::test]
async fn skeleton_card_variant_renders_multiple_elements(client: Client) -> Result<()> {
    client.goto("/story/skeleton/card").await?;
    tokio::time::sleep(std::time::Duration::from_millis(500)).await;

    let skeletons = client.find_all(By::Css("main div.animate-pulse")).await?;
    assert!(
        skeletons.len() >= 3,
        "Card variant should render at least 3 skeleton elements, found {}",
        skeletons.len()
    );
    Ok(())
}
