use binance::api::*;
use binance::account::*;
use binance::account::TimeInForce::GTC;
use binance::market::*;
use binance::errors::Error;
use binance::model::*;

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

    pub async fn place_order(&self, symbol: &str, quantity: f64, price: f64, stop_price: Option<f64>, side: &str, order_type: &str) -> Result<Transaction, Error> {
        let order_side = match side {
            "BUY" => OrderSide::Buy,
            "SELL" => OrderSide::Sell,
            _ => return Err(Error::Msg("Invalid order side".to_string())),
        };

        let order_type = match order_type {
            "LIMIT" => OrderType::Limit,
            "MARKET" => OrderType::Market,
            "STOP_LIMIT" => OrderType::StopLossLimit,
            _ => return Err(Error::Msg("Invalid order type".to_string())),
        };

        let new_order = self.account.custom_order(
            symbol,
            quantity,
            price,
            stop_price,
            order_side,
            order_type,
            GTC,
            None,
        ).unwrap();

        Ok(new_order)
    }

    pub async fn cancel_order(&self, symbol: &str, order_id: u64) -> Result<OrderCanceled, Error> {
        self.account.cancel_order(symbol, order_id)
    }

    pub async fn market_buy(&self, symbol: &str, quantity: f64) -> Result<Transaction, Error> {
        self.account.market_buy(symbol, quantity)
    }

    pub async fn market_sell(&self, symbol: &str, quantity: f64) -> Result<Transaction, Error> {
        self.account.market_sell(symbol, quantity)
    }

    pub async fn stop_limit_buy_order(&self, symbol: &str, quantity: f64, price: f64, stop_price: f64) -> Result<Transaction, Error> {
        self.account.stop_limit_buy_order(symbol, quantity, price, stop_price, GTC)
    }

    pub async fn stop_limit_sell_order(&self, symbol: &str, quantity: f64, price: f64, stop_price: f64) -> Result<Transaction, Error> {
        self.account.stop_limit_sell_order(symbol, quantity, price, stop_price, GTC)
    }

    pub async fn get_order_status(&self, symbol: &str, order_id: u64) -> Result<Order, Error> {
        self.account.order_status(symbol, order_id)
    }
}