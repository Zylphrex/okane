use thirtyfour_sync::prelude::*;

use crate::driver;

pub struct EnbridgeScraper<'a> {
    driver: &'a driver::OkaneDriver,
    username: &'a String,
    password: &'a String,
}

impl<'a> EnbridgeScraper<'a> {
    pub fn new(
        driver: &'a driver::OkaneDriver,
        username: &'a String,
        password: &'a String
    ) -> EnbridgeScraper<'a> {
        EnbridgeScraper {
            driver: driver,
            username: username,
            password: password,
        }
    }

    pub fn run(self) -> WebDriverResult<f64> {
        self.visit_enbridge_login()?;
        self.login()?;
        let balance = self.balance()?;

        let balance_str = match balance.strip_prefix("$") {
            Some(x) => x.to_string(),
            None => balance,
        };

        Ok(balance_str.parse().unwrap())
    }

    fn visit_enbridge_login(&self) -> WebDriverResult<()> {
        println!("visiting enbridge page");
        self.driver.get("https://myaccount.enbridgegas.com/")
    }

    fn login(&self) -> WebDriverResult<()> {
        println!("typing username");
        self.driver.type_text("#signin-username", self.username)?;
        println!("typing password");
        self.driver.type_text("#signin-password", self.password)?;
        println!("signing in");
        self.driver.click_element(".submit-signin")?;
        println!("dismissing notification");
        match self.driver.wait("#cancelNotification") {
            Ok(_) => self.driver.click_element("#cancelNotification")?,
            _ => (),
        }
        Ok(())
    }

    fn balance(&self) -> WebDriverResult<String> {
        println!("getting amount");
        self.driver.get_text(".account-balance-container .account-balance-details h2 b")
    }
}
