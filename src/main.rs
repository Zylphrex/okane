use std::error::Error;
use std::fmt;

use structopt::StructOpt;
use thirtyfour_sync;
use tokio;
use twilio;

mod driver;
mod scraper;

#[derive(StructOpt)]
struct Cli {
    #[structopt(long)]
    server: String,
    #[structopt(long)]
    headless: bool,

    #[structopt(long, requires_all = &["callee", "twilio-auth-token", "twilio-auth-token"])]
    twilio: bool,
    #[structopt(long, requires = "twilio")]
    caller: String,
    #[structopt(long, requires = "twilio")]
    callee: String,
    #[structopt(long, requires = "twilio")]
    twilio_account_id: String,
    #[structopt(long, requires = "twilio")]
    twilio_auth_token: String,

    #[structopt(long, requires_all = &["rogers-username", "rogers-password"])]
    rogers: bool,
    #[structopt(long, requires = "rogers")]
    rogers_username: Option<String>,
    #[structopt(long, requires = "rogers")]
    rogers_password: Option<String>,

    #[structopt(long, requires_all = &["rogers-business-username", "rogers-business-password"])]
    rogers_business: bool,
    #[structopt(long, requires = "rogers-business")]
    rogers_business_username: Option<String>,
    #[structopt(long, requires = "rogers-business")]
    rogers_business_password: Option<String>,

    #[structopt(long, requires_all = &["enbridge-username", "enbridge-password"])]
    enbridge: bool,
    #[structopt(long, requires = "enbridge")]
    enbridge_username: Option<String>,
    #[structopt(long, requires = "enbridge")]
    enbridge_password: Option<String>,

    #[structopt(long, requires_all = &["toronto-hydro-username", "toronto-hydro-password"])]
    toronto_hydro: bool,
    #[structopt(long, requires = "toronto-hydro")]
    toronto_hydro_username: Option<String>,
    #[structopt(long, requires = "toronto-hydro")]
    toronto_hydro_password: Option<String>,
}

#[derive(Debug)]
enum OkaneError {
    WebDriverError(thirtyfour_sync::error::WebDriverError),
    TwilioError(twilio::TwilioError),
}

impl fmt::Display for OkaneError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "There is an error: {}", 0)
    }
}

impl Error for OkaneError {}

#[tokio::main]
async fn main() -> Result<(), OkaneError> {
    let args = Cli::from_args();

    let mut caps = thirtyfour_sync::DesiredCapabilities::chrome();
    if args.headless {
        caps.set_headless().map_err(|err| OkaneError::WebDriverError(err))?;
    }

    let driver = driver::OkaneDriver::new(&args.server, &caps).map_err(|err| OkaneError::WebDriverError(err))?;

    let mut balances = vec!["💸 Unpaid bills".to_string()];

    if args.rogers {
        let username = &args.rogers_username.unwrap();
        let password = &args.rogers_password.unwrap();

        let rogers_scraper = scraper::rogers::RogersScraper::new(&driver, &false, username, password);
        let balance = rogers_scraper.run().map_err(|err| OkaneError::WebDriverError(err))?;
        println!("rogers balance {:?}", balance);
        if balance > 0.0 {
            balances.push(format!("- Rogers: ${:.2}", balance));
        }
    }

    if args.rogers_business {
        let username = &args.rogers_business_username.unwrap();
        let password = &args.rogers_business_password.unwrap();

        let rogers_scraper = scraper::rogers::RogersScraper::new(&driver, &true, username, password);
        let balance = rogers_scraper.run().map_err(|err| OkaneError::WebDriverError(err))?;
        println!("rogers business balance {:?}", balance);
        if balance > 0.0 {
            balances.push(format!("- Rogers Business: ${:.2}", balance));
        }
    }

    if args.enbridge {
        let username = &args.enbridge_username.unwrap();
        let password = &args.enbridge_password.unwrap();

        let enbridge_scraper = scraper::enbridge::EnbridgeScraper::new(&driver, username, password);
        let balance = enbridge_scraper.run().map_err(|err| OkaneError::WebDriverError(err))?;
        println!("enbridge balance {:?}", balance);
        if balance > 0.0 {
            balances.push(format!("- Enbridge: ${:.2}", balance));
        }
    }

    if args.toronto_hydro {
        let username = &args.toronto_hydro_username.unwrap();
        let password = &args.toronto_hydro_password.unwrap();

        let toronto_hydro_scraper = scraper::toronto_hydro::TorontoHydroScraper::new(&driver, username, password);
        let balance = toronto_hydro_scraper.run().map_err(|err| OkaneError::WebDriverError(err))?;
        println!("toronto hydro balance {:?}", balance);
        if balance > 0.0 {
            balances.push(format!("- Toronto Hydro: ${:.2}", balance));
        }
    }

    if args.twilio && balances.len() > 1 && args.callee.len() > 0 {
        let sender = &args.caller;
        let receiver = &args.callee;
        let account_id = &args.twilio_account_id;
        let auth_token = &args.twilio_auth_token;
        let twilio_client = twilio::Client::new(account_id, auth_token);

        let message = balances.join("\n");

        println!("message: {} sent from {} to {}", message, sender, receiver);
        let envelope = twilio::OutboundMessage::new(sender, receiver, &message);
        twilio_client.send_message(envelope).await.map_err(|err| OkaneError::TwilioError(err))?;
    }

    driver.quit().map_err(|err| OkaneError::WebDriverError(err))
}
