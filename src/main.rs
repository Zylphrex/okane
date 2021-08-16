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
    caller: String,
    #[structopt(long, requires = "caller")]
    callee: String,
    #[structopt(long, requires = "caller")]
    twilio_account_id: String,
    #[structopt(long, requires = "caller")]
    twilio_auth_token: String,

    #[structopt(long, requires_all = &["rogers-username", "rogers-password"])]
    rogers: bool,
    #[structopt(long, requires = "rogers")]
    rogers_username: Option<String>,
    #[structopt(long, requires = "rogers")]
    rogers_password: Option<String>,

    #[structopt(long, requires_all = &["enbridge-username", "enbridge-password"])]
    enbridge: bool,
    #[structopt(long, requires = "enbridge")]
    enbridge_username: Option<String>,
    #[structopt(long, requires = "enbridge")]
    enbridge_password: Option<String>,
}

#[tokio::main]
async fn main() {
    let args = Cli::from_args();

    let mut caps = thirtyfour_sync::DesiredCapabilities::chrome();
    if args.headless {
        caps.set_headless().ok();
    }

    match driver::OkaneDriver::new(&args.server, &caps) {
        Ok(driver) => {
            let mut balances = vec!["💸 Unpaid bills".to_string()];

            if args.rogers {
                let username = &args.rogers_username.unwrap();
                let password = &args.rogers_password.unwrap();

                let rogers_scraper = scraper::rogers::RogersScraper::new(&driver, username, password);
                match rogers_scraper.run() {
                    Ok(balance) => {
                        println!("rogers balance {:?}", balance);
                        if balance > 0.0 {
                            balances.push(format!("- Rogers: ${:.2}", balance));
                        }
                    },
                    _ => (),
                }
            }

            if args.enbridge {
                let username = &args.enbridge_username.unwrap();
                let password = &args.enbridge_password.unwrap();

                let enbridge_scraper = scraper::enbridge::EnbridgeScraper::new(&driver, username, password);
                match enbridge_scraper.run() {
                    Ok(balance) => {
                        println!("enbridge balance {:?}", balance);
                        if balance > 0.0 {
                            balances.push(format!("- Enbridge: ${:.2}", balance));
                        }
                    },
                    _ => (),
                }
            }

            if balances.len() > 1 && args.callee.len() > 0 {
                let sender = &args.caller;
                let receiver = &args.callee;
                let account_id = &args.twilio_account_id;
                let auth_token = &args.twilio_auth_token;
                let twilio_client = twilio::Client::new(account_id, auth_token);

                let message = balances.join("\n");

                println!("message: {} sent from {} to {}", message, sender, receiver);
                let envelope = twilio::OutboundMessage::new(sender, receiver, &message);
                twilio_client.send_message(envelope).await.ok();
            }

            driver.quit().ok();
        },
        _ => (),
    }
}
