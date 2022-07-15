use crate::kis::{self, api::KisApi};
use reqwest::blocking;
use std::collections::HashMap;
use std::fs::File;

type TradeResult<T> = Result<T, Box<dyn std::error::Error>>;

trait Strategy {
    fn init_kis_api(&self, config: kis::AccountConfig) {
        let kis = KisApi::new(config);
    }
    //BufReader::new(File::open(filename)?
    fn get_list_from_csv(&self, csv_name: &str, column_name: &str) -> TradeResult<Vec<String>> {
        let mut result = Vec::new();
        result.push(String::from("TEST"));

        let mut csv = csv::Reader::from_reader(File::open(csv_name)?);

        let index = csv.headers()?.iter().position(|x| x == column_name);
        let index = if let Some(n) = index { n } else { 0 };
        for record in csv.records() {
            let rec = record?;
            // println!("{}", &record[index]);
            result.push(rec[index].to_string());
        }
        Ok(result)
    }

    fn check_risk_points(&self) -> HashMap<String, i32> {
        let riskmap: HashMap<String, i32> = HashMap::new();

        riskmap
    }

    fn get_list_stocks_to_buy(&self) -> TradeResult<Vec<String>>;
    fn calculate_order_price(&self, stock: &str) -> OrderPrice;
    fn run();
}

struct OrderPrice {
    ticker: String,
    buy: u32,
    sell: u32,
}

/// Box range of price
pub struct SimpleTrade {
    stock_order_list: Vec<OrderPrice>,
}

impl SimpleTrade {
    pub fn new() -> Self {
        SimpleTrade {
            stock_order_list: Vec::default(),
        }
    }
}

impl Strategy for SimpleTrade {
    fn get_list_stocks_to_buy(&self) -> TradeResult<Vec<String>> {
        let client = reqwest::blocking::Client::new();

        let url = "http://127.0.0.1:8080/macro/vix";
        let res: blocking::Response = client
            .get(url)
            // .headers(req.headers)
            // .query(&req.parameters)
            .send()?;
        todo!()
    }

    fn calculate_order_price(&self, stock: &str) -> OrderPrice {
        todo!()
    }

    fn run() {
        todo!()
    }
}

mod unit_test {
    use crate::kis::api::*;
    use crate::kis::load_account_config;

    use super::*;

    // static TICKER: &'static str = "003490";

    fn setup_for_wrapper_api() -> KisApi {
        let mut kis = KisApi::new(load_account_config("./secret", false).unwrap());
        let res = kis.issue_access_token();
        assert_eq!(res.unwrap(), true);
        kis
    }

    #[test]
    fn test_get_list_from_csv() {
        println!("{}", std::env::current_dir().unwrap().display());
        let strategy = SimpleTrade::new();
        let result = strategy.get_list_from_csv("./data/all_latte_test.csv", "TICKER");
        if let Ok(list) = result {
            println!("{list:?}");
        } else {
            println!("{result:?}");
        }
    }
}
