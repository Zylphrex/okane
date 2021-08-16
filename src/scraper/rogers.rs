use thirtyfour_sync::prelude::*;

use crate::driver;

pub struct RogersScraper<'a> {
    driver: &'a driver::OkaneDriver,
    business: &'a bool,
    username: &'a String,
    password: &'a String,
}

impl<'a> RogersScraper<'a> {
    pub fn new(
        driver: &'a driver::OkaneDriver,
        business: &'a bool,
        username: &'a String,
        password: &'a String
    ) -> RogersScraper<'a> {
        RogersScraper {
            driver: driver,
            business: business,
            username: username,
            password: password,
        }
    }

    pub fn run(self) -> WebDriverResult<f64> {
        let balance = match *self.business {
            true => {
                self.visit_rogers_business_login()?;
                self.login_business()?;
                "$0.00".to_string()
            },
            false => {
                self.visit_rogers_login()?;
                self.login()?;
                self.balance()?
            },
        };

        let balance_str = match balance.strip_prefix("$") {
            Some(x) => x.to_string(),
            None => balance,
        };

        Ok(balance_str.parse().unwrap())
    }

    fn visit_rogers_business_login(&self) -> WebDriverResult<()> {
        println!("visiting rogers business page");
        self.driver.get("https://www.rogers.com/web/smb/bss/#/signin")
    }

    fn login_business(&self) -> WebDriverResult<()> {
        println!("typing username");
        self.driver.type_text("#USER", self.username)?;
        println!("typing password");
        self.driver.type_text("#password", self.password)?;
        println!("signing in");
        self.driver.click_element(".btn-signin button")?;
        Ok(())
    }

    fn visit_rogers_login(&self) -> WebDriverResult<()> {
        println!("visiting rogers page");
        self.driver.get("https://www.rogers.com/consumer/profile/signin")
    }

    fn login(&self) -> WebDriverResult<()> {
        println!("typing username");
        self.driver.type_text("#username", self.username)?;
        println!("typing password");
        self.driver.type_text("#password", self.password)?;
        println!("signing in");
        self.driver.click_element(".signInButton button")?;
        Ok(())
    }

    fn balance(&self) -> WebDriverResult<String> {
        println!("getting amount");
        self.driver.get_text(".amount")
    }
}
