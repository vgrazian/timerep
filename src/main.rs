use std::error::Error;
use thirtyfour::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    // define the browser options
    let mut caps = DesiredCapabilities::chrome();
    // to run Chrome in headless mode
    caps.add_arg("--headless=new")?; // comment out in development
    // initialize a driver to control a Chrome instance
    // with the specified options
    let driver = WebDriver::new("http://localhost:59631", caps).await?;

    // visit the target page
    driver.goto("https://time.ibm.com/week/").await?;
    // retrieve the source HTML of the target page
    // and print it
    let html = driver.source().await?;
    println!("{html}");

    // close the browser and release its resources
    driver.quit().await?;

    Ok(())
}
