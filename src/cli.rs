#![allow(dead_code)]
#![allow(unused_variables)]

use clap::{Arg, Command};

use trade_lib::kis;

type MyResult<T> = Result<T, Box<dyn std::error::Error>>;

pub fn get_args() -> MyResult<kis::AccountConfig> {
    let matches = Command::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(format!("\tby {}", env!("CARGO_PKG_AUTHORS").replace(":", ", ")).as_str())
        .about("stock trading sw using KIS API 한국투자증권")
        .arg(
            Arg::new("account_config_path")
                .value_name("ACCOUNT CONF FOLDER")
                .short('s')
                .help("account configuration folder for KIS connection")
                .default_value("./secret")
                .takes_value(true),
        )
        .arg(
            Arg::new("kis_server")
                .value_name("Enable 실전 투자")
                .short('t')
                .long("type")
                .help("account configuration folder for KIS connection")
                .required(true)
                .takes_value(false),
        )
        .get_matches();

    let conf_path = matches.value_of("account_config_path").unwrap();

    kis::account::load_account_config(conf_path, false)
    // Err("err".into())
}

pub fn run(config: kis::AccountConfig) -> MyResult<()> {
    println!("{config:#?}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_account_config_ok() {
        assert!(kis::load_account_config("./secret", false).is_ok());
    }

    #[test]
    #[should_panic(expected = "No such file or directory (os error 2)")]
    fn test_load_account_config_invalid_path() {
        match kis::load_account_config("./invalid_path", false) {
            Ok(_) => (),
            Err(e) => panic!("{}", e.to_string()),
        }
    }
}
