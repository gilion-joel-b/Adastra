use serde::Serialize;

#[derive(Serialize)]
pub struct Stock{
    ticker: String,
    name: String,
    price: f64,
}

impl Stock {
    pub fn new(ticker: String, name: String, price: f64) -> Stock {
        Stock {
            ticker,
            name,
            price,
        }
    }
    
}
