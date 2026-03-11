use doco::{Client, Result};
use thirtyfour::By;

/// Select component renders a trigger and opens a listbox on click.
#[doco::test]
async fn select_opens_listbox_on_click(client: Client) -> Result<()> {
    client.goto("/story/select").await?;
    tokio::time::sleep(std::time::Duration::from_millis(500)).await;

    let trigger = client.find(By::Css("main button[role='combobox']")).await?;
    trigger.click().await?;
    tokio::time::sleep(std::time::Duration::from_millis(300)).await;

    let listbox = client.find(By::Css("[role='listbox']")).await?;
    assert!(listbox.is_displayed().await?, "listbox should be visible");

    let options = listbox.find_all(By::Css("[role='option']")).await?;
    assert!(
        !options.is_empty(),
        "listbox should contain at least one option"
    );
    Ok(())
}
