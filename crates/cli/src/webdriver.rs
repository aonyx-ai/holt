//! WebDriver setup and configuration

use thirtyfour::prelude::*;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum WebDriverSetupError {
    #[error("Failed to set up WebDriver: {0}")]
    SetupFailed(#[from] thirtyfour::error::WebDriverError),
}

/// Set up a WebDriver connection to geckodriver
///
/// Configures headless mode in CI environments.
pub async fn setup_webdriver() -> Result<WebDriver, WebDriverSetupError> {
    println!("Connecting to WebDriver...");
    let mut caps = DesiredCapabilities::firefox();

    // Run headless in CI (no display server available)
    let is_ci = std::env::var("CI").is_ok();
    if is_ci {
        caps.set_headless()?;
        println!("Running Firefox in headless mode");
    }

    let driver = WebDriver::new("http://localhost:4444", caps).await?;

    // Set viewport size for consistent screenshots
    driver.set_window_rect(0, 0, 1280, 720).await?;

    Ok(driver)
}
