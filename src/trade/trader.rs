use crate::kis::{self, api::KisApi};
use reqwest::blocking;
use std::collections::HashMap;

struct OrderPrice {
    ticker: String,
    buy: u32,
    sell: u32,
}

/// Box range of price
struct SimpleTrade {
    stock_order_list: Vec<OrderPrice>,
}

impl SimpleTrade {}

impl Strategy for SimpleTrade {
    fn get_list_stocks_to_buy(&self) -> Vec<String> {
        let client = reqwest::blocking::Client::new();
        let res: blocking::Response;
        let url = "http://127.0.0.1:8080/macro/vix";
        res = client
            .get(url)
            // .headers(req.headers)
            // .query(&req.parameters)
            .send()
            .unwrap();

        todo!()
    }

    fn calculate_order_price(&self, stock: &str) -> OrderPrice {
        todo!()
    }

    fn run() {
        todo!()
    }
}

trait Strategy {
    fn init_kis_api(&self, config: kis::AccountConfig) {
        let mut kis = KisApi::new(config);
    }

    fn check_risk_points(&self) -> HashMap<String, i32> {
        let riskmap: HashMap<String, i32> = HashMap::new();

        riskmap
    }

    fn get_list_stocks_to_buy(&self) -> Vec<String>;
    fn calculate_order_price(&self, stock: &str) -> OrderPrice;
    fn run();
}

mod unit_test {
    use crate::kis::api::*;
    use crate::kis::load_account_config;

    static TICKER: &'static str = "003490";

    fn setup_for_wrapper_api() -> KisApi {
        let mut kis = KisApi::new(load_account_config("./secret", false).unwrap());
        let res = kis.issue_access_token();
        assert_eq!(res.unwrap(), true);
        kis
    }
}
