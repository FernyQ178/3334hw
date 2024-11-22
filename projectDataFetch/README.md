# Pricing Fetcher

This Rust application fetches and saves real-time pricing data for Bitcoin, Ethereum, and the S&P 500 index using public APIs.

## Features
- Fetches **Bitcoin** price data from [CoinDesk API](https://www.coindesk.com/coindesk-api).
- Fetches **Ethereum** price data from [CoinGecko API](https://www.coingecko.com/en/api).
- Fetches **S&P 500** index data from [Yahoo Finance's Unofficial API](https://query1.finance.yahoo.com/).
- Saves the fetched data to JSON files:
  - `bitcoin_price.json`
  - `ethereum_price.json`
  - `sp500_data.json`
- Runs indefinitely with a 10-second pause between fetch cycles.

## Requirements
- Rust 1.70+ installed.
- Internet connection for API requests.

## Running the Application
1. Clone this repository:
   ```bash
   git clone https://github.com/your-repo/pricing-fetcher.git
   cd pricing-fetcher
