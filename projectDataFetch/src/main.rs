use serde::{Deserialize, Serialize};
use std::{fs::File, io::Write, thread, time};

// Struct Definitions
#[derive(Serialize, Deserialize, Debug)]
struct Bitcoin {
    time: String,
    usd_rate: f64,
}

#[derive(Serialize, Deserialize, Debug)]
struct Ethereum {
    usd_rate: f64,
}

#[derive(Serialize, Deserialize, Debug)]
struct SP500 {
    value: f64,
}

// Trait Definition
trait Pricing {
    fn fetch_price(&self) -> Result<String, Box<dyn std::error::Error>>;
    fn save_to_file(&self, data: &str) -> std::io::Result<()>;
}

// Implementation for Bitcoin
impl Pricing for Bitcoin {
    fn fetch_price(&self) -> Result<String, Box<dyn std::error::Error>> {
        let response = ureq::get("https://api.coindesk.com/v1/bpi/currentprice/BTC.json")
            .call()?
            .into_string()?;

        let parsed: serde_json::Value = serde_json::from_str(&response)?;
        let time = parsed["time"]["updated"].as_str().unwrap_or_default().to_string();
        let usd_rate = parsed["bpi"]["USD"]["rate_float"].as_f64().unwrap_or(0.0);

        let bitcoin = Bitcoin { time, usd_rate };
        Ok(serde_json::to_string(&bitcoin)?) // Serialize struct to JSON for saving
    }

    fn save_to_file(&self, data: &str) -> std::io::Result<()> {
        let mut file = File::create("bitcoin_price.json")?;
        file.write_all(data.as_bytes())?;
        Ok(())
    }
}

// Implementation for Ethereum
impl Pricing for Ethereum {
    fn fetch_price(&self) -> Result<String, Box<dyn std::error::Error>> {
        let response = ureq::get("https://api.coingecko.com/api/v3/simple/price?ids=ethereum&vs_currencies=usd")
            .call()?
            .into_string()?;

        let parsed: serde_json::Value = serde_json::from_str(&response)?;
        let usd_rate = parsed["ethereum"]["usd"].as_f64().unwrap_or(0.0);

        let ethereum = Ethereum { usd_rate };
        Ok(serde_json::to_string(&ethereum)?) // Serialize struct to JSON for saving
    }

    fn save_to_file(&self, data: &str) -> std::io::Result<()> {
        let mut file = File::create("ethereum_price.json")?;
        file.write_all(data.as_bytes())?;
        Ok(())
    }
}

// Implementation for SP500
impl Pricing for SP500 {
    fn fetch_price(&self) -> Result<String, Box<dyn std::error::Error>> {
        let response = ureq::get("https://query1.finance.yahoo.com/v8/finance/chart/^GSPC")
            .call()?
            .into_string()?;

        let parsed: serde_json::Value = serde_json::from_str(&response)?;
        if let Some(close_values) = parsed["chart"]["result"][0]["indicators"]["quote"][0]["close"].as_array() {
            if let Some(latest_close) = close_values.last().and_then(|v| v.as_f64()) {
                let sp500 = SP500 { value: latest_close };
                return Ok(serde_json::to_string(&sp500)?);
            }
        }

        Err("Failed to parse S&P 500 data".into())
    }

    fn save_to_file(&self, data: &str) -> std::io::Result<()> {
        let mut file = File::create("sp500_data.json")?;
        file.write_all(data.as_bytes())?;
        Ok(())
    }
}

// Main Function
fn main() {
    let bitcoin = Bitcoin {
        time: String::new(),
        usd_rate: 0.0,
    };
    let ethereum = Ethereum { usd_rate: 0.0 };
    let sp500 = SP500 { value: 0.0 };

    let assets: Vec<&dyn Pricing> = vec![&bitcoin, &ethereum, &sp500];

    println!("Starting pricing fetcher...");
    loop {
        for asset in &assets {
            match asset.fetch_price() {
                Ok(data) => {
                    println!("Fetched data: {}", data);
                    if let Err(e) = asset.save_to_file(&data) {
                        eprintln!("Failed to save data: {}", e);
                    }
                }
                Err(e) => eprintln!("Failed to fetch data: {}", e),
            }
        }
        println!("Waiting for 10 seconds...");
        thread::sleep(time::Duration::from_secs(10));
    }
}