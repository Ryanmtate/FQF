use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::stock_data::{Returns, StockData};

pub type Ticker = String;

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Asset {
    // The amount invested into the asset
    pub amount_invested: f64,
    // The ticker of the asset (or cusip), e.g. AAPL
    pub ticker: Ticker,
    // Returns for the asset
    pub stock_data: StockData,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Portfolio {
    // This amount is the initial value of the portfolio that will be invested (or allocated) across the assets.
    pub initial_value: f64,
    // Date of initial value
    pub initial_date: chrono::DateTime<Utc>,
    // Assets
    pub assets: HashMap<Ticker, Asset>,
}

impl Portfolio {
    pub fn new(initial_value: f64, initial_date: Option<DateTime<Utc>>) -> Self {
        Self {
            initial_value,
            initial_date: initial_date.unwrap_or(Utc::now()),
            assets: HashMap::new(),
        }
    }

    pub async fn download_asset_data(
        &mut self,
        tickers: Vec<Ticker>,
        num_days: usize,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let stock_data = StockData::download(tickers, num_days).await?;

        for s_data in stock_data {
            for data in s_data.data {
                let asset = self.assets.entry(data.symbol.clone()).or_default();

                asset.stock_data.data.push(data);
            }
        }

        for (_, asset) in self.assets.iter_mut() {
            asset.stock_data.data.sort_by(|a, b| a.date.cmp(&b.date));
        }

        Ok(())
    }

    // Save portfolio as JSON file
    pub fn save(&self, file: std::path::PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        let file = std::fs::File::create(file)?;
        let writer = std::io::BufWriter::new(file);
        serde_json::to_writer(writer, &self).map_err(|e| e.into())
    }

    // Load portfolio from JSON file
    pub fn load(file: std::path::PathBuf) -> Result<Self, Box<dyn std::error::Error>> {
        let file = std::fs::File::open(file)?;
        let reader = std::io::BufReader::new(file);
        serde_json::from_reader(reader).map_err(|e| e.into())
    }
}
