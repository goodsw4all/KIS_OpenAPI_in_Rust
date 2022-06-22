use super::account::AccountConfig;

use serde_json::json;
use tungstenite::{connect, Message};
use url::Url;

use std::sync::mpsc::Sender;
use std::{sync::mpsc, thread};

type KisResult<T> = Result<T, Box<dyn std::error::Error>>;

fn websoket_test(conf: &AccountConfig, ticker: &str, tx: Sender<String>) {
    let params = json!({
      "header": {
        "appkey": conf.get_apikey(),
        "appsecret": conf.get_secret(),
        "custtype": "P",
        "tr_type": "1",
        "content-type": "utf-8"
      },
      "body": {
        "input": {
          "tr_id": "H0STASP0",
          "tr_key": ticker
        }
      }
    });

    println!("Req Header {} \nBody {}", params["header"], params["body"]);

    // let url = 'ws://ops.koreainvestment.com:21000' //실전투자
    let url = "ws://ops.koreainvestment.com:31000"; //모의투자
    let (mut socket, response) = connect(Url::parse(url).unwrap()).expect("Can't connect");

    println!("Connected to the server");
    println!("Response HTTP code: {}", response.status());
    println!("Response contains the following headers:");

    for (ref header, _value) in response.headers() {
        println!("* {:?}", header);
    }

    socket
        .write_message(Message::Text(params.to_string()))
        .unwrap();

    loop {
        println!("Loop In");
        let msg = socket.read_message().expect("Error reading message");
        println!("Received: {}", msg);
        // ?tx.send(format!("Received: {}", msg));
    }
    // socket
}

#[cfg(test)]
mod unit_websoket {
    use std::{sync::mpsc, thread};

    use crate::kis::load_account_config;

    use super::websoket_test;

    #[test]
    #[ignore]
    fn test_ws_connect() {
        let (tx, rx) = mpsc::channel::<String>();
        let conf = load_account_config("./secret", false).unwrap();

        let thread_join_handle = thread::spawn(move || {
            // some work herewebsoket_test(&conf, "005935");
            websoket_test(&conf, "005935", tx);
        });

        let received = rx.recv().unwrap();
        println!("Got: {}", received);

        let res = thread_join_handle.join();
    }
}
