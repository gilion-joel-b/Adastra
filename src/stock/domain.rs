use serde::Serialize;

#[derive(Serialize)]
pub struct Stock{
    ticker: String,
    price: f64,
}

impl Stock {
    pub fn new(ticker: String, price: f64) -> Stock {
        Stock {
            ticker,
            price,
        }
    }
}

