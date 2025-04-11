use serde::Deserialize;
use std::{fs::OpenOptions, io::Write, thread, time::Duration};

trait Pricing {
    fn fetch_price(&mut self) -> Result<(), String>;
    fn save_to_file(&self) -> Result<(), String>;
}

#[derive(Debug, Deserialize)]
struct CoinResponse {
    bitcoin: Option<CoinPrice>,
    ethereum: Option<CoinPrice>,
}

#[derive(Debug, Deserialize)]
struct CoinPrice {
    usd: f64,
}

#[derive(Debug)]
struct Bitcoin {
    price: f64,
}

#[derive(Debug)]
struct Ethereum {
    price: f64,
}

#[derive(Debug, Deserialize)]
struct SP500Response {
    price: f64,
}

#[derive(Debug)]
struct SP500 {
    price: f64,
}

// Trait Implementation

impl Pricing for Bitcoin {
    fn fetch_price(&mut self) -> Result<(), String> {
        let url = "https://api.coingecko.com/api/v3/simple/price?ids=bitcoin&vs_currencies=usd";
        let response: CoinResponse = ureq::get(url)
            .call()
            .map_err(|e| e.to_string())?
            .into_json()
            .map_err(|e| e.to_string())?;

        if let Some(data) = response.bitcoin {
            self.price = data.usd;
            Ok(())
        } else {
            Err("Missing Bitcoin data".to_string())
        }
    }

    fn save_to_file(&self) -> Result<(), String> {
        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open("bitcoin.txt")
            .map_err(|e| e.to_string())?;
        writeln!(file, "Bitcoin price: ${}", self.price).map_err(|e| e.to_string())
    }
}

impl Pricing for Ethereum {
    fn fetch_price(&mut self) -> Result<(), String> {
        let url = "https://api.coingecko.com/api/v3/simple/price?ids=ethereum&vs_currencies=usd";
        let response: CoinResponse = ureq::get(url)
            .call()
            .map_err(|e| e.to_string())?
            .into_json()
            .map_err(|e| e.to_string())?;

        if let Some(data) = response.ethereum {
            self.price = data.usd;
            Ok(())
        } else {
            Err("Missing Ethereum data".to_string())
        }
    }

    fn save_to_file(&self) -> Result<(), String> {
        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open("ethereum.txt")
            .map_err(|e| e.to_string())?;
        writeln!(file, "Ethereum price: ${}", self.price).map_err(|e| e.to_string())
    }
}
//Api not working couldn't find a free api to solve this issue getting error for now
impl Pricing for SP500 {
    fn fetch_price(&mut self) -> Result<(), String> {
        let url = "https://financialmodelingprep.com/api/v3/quote/%5EGSPC?apikey=kUtw8w04QHypaQP4iST1fq2iQ7swUnlL";
        let response: Vec<SP500Response> = ureq::get(url)
            .call()
            .map_err(|e| e.to_string())?
            .into_json()
            .map_err(|e| e.to_string())?;

        if let Some(data) = response.first() {
            self.price = data.price;
            Ok(())
        } else {
            Err("Missing SP500 data".to_string())
        }
    }

    fn save_to_file(&self) -> Result<(), String> {
        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open("sp500.txt")
            .map_err(|e| e.to_string())?;
        writeln!(file, "S&P 500 index: {}", self.price).map_err(|e| e.to_string())
    }
}

// Main function

fn main() {
    let mut bitcoin = Bitcoin { price: 0.0 };
    let mut ethereum = Ethereum { price: 0.0 };
    let mut sp500 = SP500 { price: 0.0 };

    loop {
        let mut assets: Vec<&mut dyn Pricing> = vec![&mut bitcoin, &mut ethereum, &mut sp500];

        for asset in assets.iter_mut() {
            if let Err(e) = asset.fetch_price() {
                eprintln!("Error fetching data: {}", e);
            }

            if let Err(e) = asset.save_to_file() {
                eprintln!("Error saving data: {}", e);
            }
        }
        println!("Data fetched and saved. Waiting 10 seconds...");
        thread::sleep(Duration::from_secs(10));
    }
}
