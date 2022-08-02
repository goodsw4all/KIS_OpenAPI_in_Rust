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
    fn get_tickers_from_csv(&self, csv_name: &str, column_name: &str) -> TradeResult<Vec<String>> {
        let mut result = Vec::new();
        result.push(String::from("TEST"));

        let mut csv = csv::Reader::from_reader(File::open(csv_name)?);

        let idx_of_ticker = csv
            .headers()?
            .iter()
            .position(|x| x == column_name);
        let index = if let Some(n) = idx_of_ticker { n } else { 0 };
        for header in csv.headers() {
            println!("{:?}", header);
        }
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

    fn make_list_stocks_to_buy_from_csv(&mut self) -> TradeResult<()>;
    fn calculate_order_price(&self, stock: &str) -> OrderPrice;
    fn trade(&self) -> TradeResult<()>;
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

    fn check_macro_signal(&self) {
        let client = reqwest::blocking::Client::new();

        let url = "http://127.0.0.1:8080/macro/status";
        let res: blocking::Response = client
            .get(url)
            // .headers(req.headers)
            // .query(&req.parameters)
            .send()
            .unwrap();
    }
}

impl Strategy for SimpleTrade {
    fn make_list_stocks_to_buy_from_csv(&mut self) -> TradeResult<()> {
        let stock_list = self.get_tickers_from_csv("./data/all_latte_test.csv", "TICKER")?;
        for stock in stock_list.iter() {
            self.stock_order_list
                .push(self.calculate_order_price(stock));
        }

        Ok(())
    }

    fn calculate_order_price(&self, stock: &str) -> OrderPrice {
        OrderPrice {
            ticker: String::from(stock),
            buy: 0,
            sell: 0,
        }
    }

    fn trade(&self) -> TradeResult<()> {
        Ok(())
    }

    fn run() {
        todo!()
    }
}

#[cfg(test)]
mod unit_test {
    use super::*;

    use crate::kis::load_account_config;

    // static TICKER: &'static str = "003490";
    fn setup() -> KisApi {
        let mut kis = KisApi::new(load_account_config("./secret", false).unwrap());
        let res = kis.issue_access_token();
        assert_eq!(res.unwrap(), true);
        kis
    }

    #[test]
    fn test_get_list_from_csv() {
        println!(
            "{}",
            std::env::current_dir()
                .unwrap()
                .display()
        );
        let strategy = SimpleTrade::new();
        let result = strategy.get_tickers_from_csv("./data/all_latte_test.csv", "TICKER");
        if let Ok(list) = result {
            println!("{list:?}");
        } else {
            println!("{result:?}");
        }
    }
}
