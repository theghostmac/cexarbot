use binance::account::TimeInForce::GTC;
use binance::account::*;
use binance::api::*;
use binance::errors::Error;
use binance::market::*;
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


    pub fn get_account_information(&self) -> Result<AccountInformation, Error> {
        let account = self.account.get_account().unwrap();
        Ok(account)
    }

    pub fn get_price(&self, symbol: &str) -> Result<f64, Error> {
        let price = self.market.get_price(symbol).unwrap().price;
        Ok(price)
    }

    pub fn place_order(
        &self,
        symbol: &str,
        quantity: f64,
        price: f64,
        stop_price: Option<f64>,
        side: &str,
        order_type: &str,
    ) -> Result<Transaction, Error> {
        let order_side = match side {
            "BUY" => OrderSide::Buy,
            "SELL" => OrderSide::Sell,
            _ => {
                return Err(Error::from_kind(binance::errors::ErrorKind::Msg(
                    "Invalid order side".to_string(),
                )))
            }
        };

        let order_type = match order_type {
            "LIMIT" => OrderType::Limit,
            "MARKET" => OrderType::Market,
            "STOP_LIMIT" => OrderType::StopLossLimit,
            _ => {
                return Err(Error::from_kind(binance::errors::ErrorKind::Msg(
                    "Invalid order type".to_string(),
                )))
            }
        };

        let new_order = self
            .account
            .custom_order(
                symbol, quantity, price, stop_price, order_side, order_type, GTC, None,
            )
            .unwrap();

        Ok(new_order)
    }

    pub fn cancel_order(&self, symbol: &str, order_id: u64) -> Result<OrderCanceled, Error> {
        self.account.cancel_order(symbol, order_id)
    }

    pub fn market_buy(&self, symbol: &str, quantity: f64) -> Result<Transaction, Error> {
        self.account.market_buy(symbol, quantity)
    }

    pub fn market_sell(&self, symbol: &str, quantity: f64) -> Result<Transaction, Error> {
        self.account.market_sell(symbol, quantity)
    }

    pub fn stop_limit_buy_order(
        &self,
        symbol: &str,
        quantity: f64,
        price: f64,
        stop_price: f64,
    ) -> Result<Transaction, Error> {
        self.account
            .stop_limit_buy_order(symbol, quantity, price, stop_price, GTC)
    }

    pub fn stop_limit_sell_order(
        &self,
        symbol: &str,
        quantity: f64,
        price: f64,
        stop_price: f64,
    ) -> Result<Transaction, Error> {
        self.account
            .stop_limit_sell_order(symbol, quantity, price, stop_price, GTC)
    }

    pub fn get_order_status(&self, symbol: &str, order_id: u64) -> Result<Order, Error> {
        self.account.order_status(symbol, order_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::secrets::*;

    fn get_client() -> BinanceClient {
        let config = Config::load().unwrap();
        let binance_api_key = config.binance_api_key;
        let binance_secret_key = config.binance_secret_key;

        BinanceClient::new(Some(binance_api_key), Some(binance_secret_key))
    }

    #[test]
    fn test_get_account_information() {
        let client = get_client();
        let account_info = client.get_account_information().unwrap();
        println!("{:?}", account_info);
        assert!(account_info.balances.len() > 0);
    }
}
