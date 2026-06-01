use doco::{Client, Result};
use thirtyfour::By;

/// Table component renders as a <table> inside a scrollable wrapper.
#[doco::test]
async fn table_renders_with_structure(client: Client) -> Result<()> {
    client.goto("/story/table").await?;
    tokio::time::sleep(std::time::Duration::from_millis(500)).await;

    let table = client.find(By::Css("main table")).await?;
    let tag = table.tag_name().await?;
    assert_eq!(
        tag.to_lowercase(),
        "table",
        "Table root element should be a <table>"
    );

    let thead = table.find(By::Css("thead")).await?;
    assert!(thead.is_displayed().await?, "thead should be visible");

    let tbody = table.find(By::Css("tbody")).await?;
    assert!(tbody.is_displayed().await?, "tbody should be visible");

    let tfoot = table.find(By::Css("tfoot")).await?;
    assert!(tfoot.is_displayed().await?, "tfoot should be visible");

    Ok(())
}

/// Table body contains the expected number of data rows.
#[doco::test]
async fn table_body_has_rows(client: Client) -> Result<()> {
    client.goto("/story/table").await?;
    tokio::time::sleep(std::time::Duration::from_millis(500)).await;

    let rows = client.find_all(By::Css("main table tbody tr")).await?;
    assert_eq!(rows.len(), 5, "table body should contain 5 invoice rows");

    Ok(())
}

/// Table caption is rendered.
#[doco::test]
async fn table_has_caption(client: Client) -> Result<()> {
    client.goto("/story/table").await?;
    tokio::time::sleep(std::time::Duration::from_millis(500)).await;

    let caption = client.find(By::Css("main table caption")).await?;
    let text = caption.text().await?;
    assert!(
        text.contains("recent invoices"),
        "caption should mention recent invoices, got: {text}"
    );

    Ok(())
}
