use clap::{Arg, Command};

fn cli() -> Command {
    Command::new("cexarbot")
        .about("An AI-powered trading bot for trading on CEXes (currently supports Binance only).")
        .version("1.0")
        .author("GhostMac")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("")
        )
}

fn main() {
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
}

