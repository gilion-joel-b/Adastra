use std::sync::Arc;

use yahoo_finance_api::{YahooConnector, YahooError};

use super::domain::Stock;

#[derive(Clone)]
pub struct StockService {
    client: Arc<YahooConnector>,
}

impl StockService {
    pub fn new(client: YahooConnector) -> Self {
        StockService {
            client: Arc::new(client),
        }
    }

    pub async fn search_stock_by_ticker(self, ticker: String) -> Result<Stock, YahooError> {
        let price = self
            .client
            .get_latest_quotes(&ticker, "1d")
            .await?
            .last_quote()?
            .close;

        self.client
            .search_ticker(&ticker)
            .await?
            .quotes
            .first()
            .map(|quote| Stock::new(quote.symbol.clone(), quote.long_name.clone(), price))
            .ok_or_else(|| YahooError::EmptyDataSet)
    }

}
