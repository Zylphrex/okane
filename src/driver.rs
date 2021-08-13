use std::time;

use serde;
use thirtyfour_sync::GenericWebDriver;
use thirtyfour_sync::error;
use thirtyfour_sync::prelude::*;
use thirtyfour_sync::http::reqwest_sync::ReqwestDriverSync;

const TIMEOUT: time::Duration = time::Duration::from_secs(120);
const INTERVAL: time::Duration = time::Duration::from_secs(1);

pub struct OkaneDriver {
    driver: GenericWebDriver<ReqwestDriverSync>,
}

impl OkaneDriver {
    pub fn new<C: serde::ser::Serialize>(server_url: &str, capabilities: &C) -> WebDriverResult<OkaneDriver> {
        let driver = WebDriver::new(server_url, &capabilities)?;

        Ok(
            OkaneDriver {
                driver: driver,
            }
        )
    }

    pub fn get(&self, url: &str) -> WebDriverResult<()> {
        self.driver.get(url)?;
        Ok(())
    }

    pub fn wait(&self, selector: &str) -> WebDriverResult<()> {
        match self.driver.query(By::Css(selector)).wait(TIMEOUT, INTERVAL).exists()? {
            true => Ok(()),
            false => Err(error::no_such_element(""))
        }
    }

    pub fn click_element(&self, selector: &str) -> WebDriverResult<()> {
        self.wait(selector)?;
        self.driver.find_element(By::Css(selector))?.click()?;
        Ok(())
    }

    pub fn type_text(&self, selector: &str, text: &str) -> WebDriverResult<()> {
        self.wait(selector)?;
        self.driver.find_element(By::Css(selector))?.send_keys(text)?;
        Ok(())
    }

    pub fn get_text(&self, selector: &str) -> WebDriverResult<String> {
        self.wait(selector)?;
        self.driver.find_element(By::Css(selector))?.text()
    }

    pub fn quit(self) -> WebDriverResult<()> {
        self.driver.quit()?;
        Ok(())
    }
}
