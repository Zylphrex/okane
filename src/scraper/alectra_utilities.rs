use thirtyfour_sync::prelude::*;

use crate::driver;

pub struct AlectraUtilitiesScraper<'a> {
    driver: &'a driver::OkaneDriver,
    username: &'a String,
    password: &'a String,
}

impl<'a> AlectraUtilitiesScraper<'a> {
    pub fn new(
        driver: &'a driver::OkaneDriver,
        username: &'a String,
        password: &'a String
    ) -> AlectraUtilitiesScraper<'a> {
        AlectraUtilitiesScraper {
            driver: driver,
            username: username,
            password: password,
        }
    }

    pub fn run(self) -> WebDriverResult<f64> {
        self.visit_alectra_utilities_login()?;
        self.login()?;
        let balance = self.balance()?;

        let balance_str = match balance.trim().strip_prefix("$") {
            Some(x) => x.to_string(),
            None => balance,
        };

        Ok(balance_str.parse().unwrap())
    }

    fn visit_alectra_utilities_login(&self) -> WebDriverResult<()> {
        println!("visiting alectra utilities page");
        self.driver.get("https://myaccount.alectrautilities.com/app/login.jsp")
    }

    fn login(&self) -> WebDriverResult<()> {
        self.driver.click_element("#accountNumberLoginTab a")?;
        println!("typing username");
        self.driver.type_text("#accessCode", self.username)?;
        println!("typing password");
        self.driver.type_text("#password2", self.password)?;
        println!("sigining in");
        self.driver.click_element("#accountLogin button")?;
        Ok(())
    }

    fn balance(&self) -> WebDriverResult<String> {
        println!("getting amount");
        self.driver.get_text("tr.hidden-xs:nth-child(2) > td:nth-child(4)")
    }
}
