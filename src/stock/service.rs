use std::sync::Arc;

use polars::prelude::*;
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

    pub async fn get_monthly_historical_prices(
        self,
        ticker: String,
    ) -> Result<Vec<yahoo_finance_api::Quote>, YahooError> {
        self.client
            .get_quote_range(&ticker, "1m", "1d")
            .await?
            .quotes()
    }

    async fn get_daily_average_price(
        self,
        ticker: String,
    ) -> Result<DataFrame, YahooError> {
        let q = self
            .client
            .get_quote_range(&ticker, "1m", "1d")
            .await?
            .quotes()?;

        let df: DataFrame = df!(
            "date" => q.iter().map(|quote| quote.timestamp).collect::<Vec<_>>(),
            "open" => q.iter().map(|quote| quote.open).collect::<Vec<_>>(),
            "close" => q.iter().map(|quote| quote.close).collect::<Vec<_>>(),
            "high" => q.iter().map(|quote| quote.high).collect::<Vec<_>>(),
            "low" => q.iter().map(|quote| quote.low).collect::<Vec<_>>()
        )
        .unwrap();

        let rolling_mean = df
            .lazy()
            .select([col("close")
                .rolling_mean(RollingOptionsFixedWindow {
                    window_size: 5,
                    min_periods: 1,
                    weights: None,
                    center: false,
                    fn_params: None,
                })
                .alias("rolling_mean")])
            .collect()
            .unwrap();

        Ok(rolling_mean)
    }
}
