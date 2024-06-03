use binance::api::*;
use binance::account::*;
use binance::market::*;
use binance::errors::Error;
use binance::model::Order;

pub struct BinanceClient {
    market: Market,
    account: Account,
}

impl BinanceClient {
    pub fn new(api_key: Option<String>, secret_key: Option<String>) -> Self {
        let market = Binance::new(api_key.clone(), secret_key.clone());
        let account = Account::new(api_key, secret_key);
        BinanceClient { market, account }
    }

    pub async fn get_price(&self, symbol: &str) -> Result<f64, Error> {
        let price = self.market.get_price(symbol).unwrap();
        Ok(price.price)
    }

    pub async fn place_order(&self, symbol: &str, side: &str, quantity: f64, price: f64) -> Result<Order, Error> {
      match side {
          "BUY" => self.market.buy_limit(symbol, quantity, price).await,
          "SELL" => self.market.sell_limit(symbol, quantity, price).await,
          _ => Err(Error::Msg("Invalid order side".to_string())),
      }
    }
}