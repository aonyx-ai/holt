use doco::{Client, Result};
use thirtyfour::By;

/// Progress component renders with the progressbar role.
#[doco::test]
async fn progress_has_progressbar_role(client: Client) -> Result<()> {
    client.goto("/story/progress").await?;
    tokio::time::sleep(std::time::Duration::from_millis(500)).await;

    let bar = client.find(By::Css("[role='progressbar']")).await?;
    let role = bar.attr("role").await?;
    assert_eq!(role.as_deref(), Some("progressbar"));
    Ok(())
}

/// Progress component exposes aria-valuemin and aria-valuemax attributes.
#[doco::test]
async fn progress_has_aria_value_bounds(client: Client) -> Result<()> {
    client.goto("/story/progress").await?;
    tokio::time::sleep(std::time::Duration::from_millis(500)).await;

    let bar = client.find(By::Css("[role='progressbar']")).await?;

    let min = bar.attr("aria-valuemin").await?;
    assert_eq!(min.as_deref(), Some("0"), "aria-valuemin should be 0");

    let max = bar.attr("aria-valuemax").await?;
    assert_eq!(max.as_deref(), Some("100"), "aria-valuemax should be 100");

    Ok(())
}

/// Progress component sets aria-valuenow to reflect the current value.
#[doco::test]
async fn progress_has_aria_valuenow(client: Client) -> Result<()> {
    client.goto("/story/progress").await?;
    tokio::time::sleep(std::time::Duration::from_millis(500)).await;

    let bar = client.find(By::Css("[role='progressbar']")).await?;
    let now = bar.attr("aria-valuenow").await?;
    assert!(
        now.is_some(),
        "aria-valuenow should be present on the progress bar"
    );

    Ok(())
}
