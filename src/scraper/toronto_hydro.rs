use thirtyfour_sync::prelude::*;

use crate::driver;

pub struct TorontoHydroScraper<'a> {
    driver: &'a driver::OkaneDriver,
    username: &'a String,
    password: &'a String,
}

impl<'a> TorontoHydroScraper<'a> {
    pub fn new(
        driver: &'a driver::OkaneDriver,
        username: &'a String,
        password: &'a String
    ) -> TorontoHydroScraper<'a> {
        TorontoHydroScraper {
            driver: driver,
            username: username,
            password: password,
        }
    }

    pub fn run(self) -> WebDriverResult<f64> {
        self.visit_toronto_hydro_login()?;
        self.login()?;
        let balance = self.balance()?;

        let balance_str = match balance.strip_prefix("$") {
            Some(x) => x.to_string(),
            None => balance,
        };

        Ok(balance_str.parse().unwrap())
    }

    fn visit_toronto_hydro_login(&self) -> WebDriverResult<()> {
        println!("visiting toronto hydro page");
        self.driver.get("https://www.torontohydro.com/log-in")
    }

    fn login(&self) -> WebDriverResult<()> {
        println!("typing username");
        self.driver.type_text("#email", self.username)?;
        println!("typing password");
        self.driver.type_text("#password", self.password)?;
        println!("signing in");
        self.driver.click_element("#th-module-authentication button")?;
        Ok(())
    }

    fn balance(&self) -> WebDriverResult<String> {
        println!("getting amount");
        self.driver.get_text(".current-balance .thc-value")
    }
}
