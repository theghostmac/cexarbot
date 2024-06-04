mod binance_client;
mod cexar_ai;
mod config;

use crate::binance_client::binance_client::BinanceClient;
use crate::cexar_ai::ai_client::{get_account_information_response, get_openai_prediction};
use clap::{Arg, Command};
use config::secrets::Config;

fn cli() -> Command {
    Command::new("cexarbot")
        .about("An AI-powered trading bot for trading on CEXes (currently supports Binance only).")
        .version("1.0")
        .author("GhostMac")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("trade")
                .about("Executes a trade using a trading strategy.")
                .arg(
                    Arg::new("strategy")
                        .short('s')
                        .long("strategy")
                        .value_name("STRATEGY")
                        .help("Trading strategy to use.")
                        .required(true),
                )
                .arg(
                    Arg::new("symbol")
                        .short('t')
                        .long("symbol")
                        .value_name("SYMBOL")
                        .help("Trading pair symbol (e.g., BTCUSDT).")
                        .required(true),
                ),
        )
        .subcommand(Command::new("account").about("Displays personal account information."))
        .subcommand(
            Command::new("backtest")
                .about("Back-testing a trading strategy.")
                .arg(
                    Arg::new("strategy")
                        .short('s')
                        .long("strategy")
                        .value_name("STRATEGY")
                        .help("The trading strategy to backtest.")
                        .required(true),
                )
                .arg(
                    Arg::new("symbol")
                        .short('t')
                        .long("symbol")
                        .value_name("SYMBOL")
                        .help("Trading pair symbol (e.g., BTCUSDT).")
                        .required(true),
                ),
        )
}

async fn execute_command(matches: clap::ArgMatches, config: Config) -> Result<(), Box<dyn std::error::Error>> {
    let openai_api_key = config.openai_api_key;
    let binance_client = BinanceClient::new(
        Some(config.binance_api_key.clone()),
        Some(config.binance_secret_key.clone()),
    );

    match matches.subcommand() {
        Some(("trade", sub_matches)) => {
            let strategy = sub_matches.get_one::<String>("strategy").unwrap();
            let symbol = sub_matches.get_one::<String>("symbol").unwrap();
            println!("Executing {} strategy on {}", strategy, symbol);

            let ai_prompt = format!(
                "Analyze the market for {} and provide a trading decision.",
                symbol
            );
            let prediction = get_openai_prediction(ai_prompt, openai_api_key).await?;
            println!("AI Prediction: {}", prediction);

            // TODO: call the function to execute the trading strategy.
        }
        Some(("backtest", sub_matches)) => {
            let strategy = sub_matches.get_one::<String>("strategy").unwrap();
            let symbol = sub_matches.get_one::<String>("symbol").unwrap();
            println!("Back-testing {} strategy on {}", strategy, symbol);
            // TODO: call the function to execute a back-test on the trading strategy.
        }
        Some(("account", _sub_matches)) => {
            let account_info = binance_client.get_account_information()?;
            println!("{:?}", account_info);
            let response = get_account_information_response(account_info, openai_api_key).await?;
            println!("{}", response);
        }
        _ => unreachable!("The CLI should require a subcommand, this should never happen"),
    }

    Ok(())
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::load().unwrap();

    println!(
        r#"

 ▄████▄  ▓█████ ▒██   ██▒ ▄▄▄       ██▀███   ▄▄▄▄    ▒█████  ▄▄▄█████▓
▒██▀ ▀█  ▓█   ▀ ▒▒ █ █ ▒░▒████▄    ▓██ ▒ ██▒▓█████▄ ▒██▒  ██▒▓  ██▒ ▓▒
▒▓█    ▄ ▒███   ░░  █   ░▒██  ▀█▄  ▓██ ░▄█ ▒▒██▒ ▄██▒██░  ██▒▒ ▓██░ ▒░
▒▓▓▄ ▄██▒▒▓█  ▄  ░ █ █ ▒ ░██▄▄▄▄██ ▒██▀▀█▄  ▒██░█▀  ▒██   ██░░ ▓██▓ ░
▒ ▓███▀ ░░▒████▒▒██▒ ▒██▒ ▓█   ▓██▒░██▓ ▒██▒░▓█  ▀█▓░ ████▓▒░  ▒██▒ ░
░ ░▒ ▒  ░░░ ▒░ ░▒▒ ░ ░▓ ░ ▒▒   ▓▒█░░ ▒▓ ░▒▓░░▒▓███▀▒░ ▒░▒░▒░   ▒ ░░
  ░  ▒    ░ ░  ░░░   ░▒ ░  ▒   ▒▒ ░  ░▒ ░ ▒░▒░▒   ░   ░ ▒ ▒░     ░
░           ░    ░    ░    ░   ▒     ░░   ░  ░    ░ ░ ░ ░ ▒    ░
░ ░         ░  ░ ░    ░        ░  ░   ░      ░          ░ ░
░                                                 ░

        "#
    );

    let matches = cli().get_matches();

    execute_command(matches, config).await

}
