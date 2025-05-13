use std::{
    env,
    fs::File,
    io::{self, BufRead},
    sync::{mpsc, Arc, Mutex},
    thread,
    time::{Duration, Instant, SystemTime},
};

use reqwest::blocking::Client;

#[derive(Debug)]
struct WebsiteStatus {
    url: String,
    action_status: Result<u16, String>,
    response_time: Duration,
    timestamp: SystemTime,
}

fn read_urls(file_path: &str) -> io::Result<Vec<String>> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);
    Ok(reader
        .lines()
        .filter_map(Result::ok)
        .filter(|line| !line.trim().is_empty() && !line.trim_start().starts_with('#'))
        .collect())
}

fn check_website(
    client: &Client,
    url: String,
    timeout: Duration,
    retries: usize,
) -> WebsiteStatus {
    let start = Instant::now();
    let mut attempts = 0;
    let mut last_err = None;

    while attempts <= retries {
        let result = client.get(&url).timeout(timeout).send();
        let timestamp = SystemTime::now();
        match result {
            Ok(response) => {
                return WebsiteStatus {
                    url,
                    action_status: Ok(response.status().as_u16()),
                    response_time: start.elapsed(),
                    timestamp,
                };
            }
            Err(e) => {
                last_err = Some(e.to_string());
                thread::sleep(Duration::from_millis(100));
            }
        }
        attempts += 1;
    }

    WebsiteStatus {
        url,
        action_status: Err(last_err.unwrap_or("Unknown error".to_string())),
        response_time: start.elapsed(),
        timestamp: SystemTime::now(),
    }
}

fn write_json_report(results: &[WebsiteStatus]) -> io::Result<()> {
    let mut json = String::from("[\n");
    for (i, r) in results.iter().enumerate() {
        let entry = format!(
            "  {{\n    \"url\": \"{}\",\n    \"status\": {},\n    \"response_time_ms\": {},\n    \"timestamp\": {:?}\n  }}{}",
            r.url,
            match &r.action_status {
                Ok(code) => code.to_string(),
                Err(e) => format!("\"{}\"", e),
            },
            r.response_time.as_millis(),
            r.timestamp,
            if i == results.len() - 1 { "\n" } else { ",\n" }
        );
        json.push_str(&entry);
    }
    json.push_str("]\n");

    std::fs::write("status.json", json)?;
    Ok(())
}

fn usage_and_exit() -> ! {
    eprintln!("Usage: website_checker [--file sites.txt] [URL ...] [--workers N] [--timeout S] [--retries N]");
    std::process::exit(2);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut file: Option<String> = None;
    let mut urls: Vec<String> = Vec::new();
    let mut workers = num_cpus::get();
    let mut timeout_secs = 5;
    let mut retries = 0;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--file" => {
                i += 1;
                if i >= args.len() {
                    usage_and_exit();
                }
                file = Some(args[i].clone());
            }
            "--workers" => {
                i += 1;
                if i >= args.len() {
                    usage_and_exit();
                }
                workers = args[i].parse().unwrap_or_else(|_| {
                    eprintln!("Invalid number of workers");
                    std::process::exit(2);
                });
            }
            "--timeout" => {
                i += 1;
                timeout_secs = args[i].parse().unwrap_or(5);
            }
            "--retries" => {
                i += 1;
                retries = args[i].parse().unwrap_or(0);
            }
            arg => {
                if arg.starts_with("--") {
                    usage_and_exit();
                }
                urls.push(arg.to_string());
            }
        }
        i += 1;
    }

    if file.is_none() && urls.is_empty() {
        usage_and_exit();
    }

    if let Some(path) = file {
        match read_urls(&path) {
            Ok(mut from_file) => urls.append(&mut from_file),
            Err(e) => {
                eprintln!("Error reading file: {}", e);
                std::process::exit(1);
            }
        }
    }

    let urls = Arc::new(Mutex::new(urls));
    let client = Arc::new(Client::builder().timeout(Duration::from_secs(timeout_secs)).build().unwrap());

    let (tx, rx) = mpsc::channel();
    let mut handles = vec![];

    for _ in 0..workers {
        let tx = tx.clone();
        let urls = Arc::clone(&urls);
        let client = Arc::clone(&client);
        let timeout = Duration::from_secs(timeout_secs);

        let handle = thread::spawn(move || {
            loop {
                let url_opt = {
                    let mut urls = urls.lock().unwrap();
                    urls.pop()
                };

                match url_opt {
                    Some(url) => {
                        let result = check_website(&client, url, timeout, retries);
                        tx.send(result).unwrap();
                    }
                    None => break,
                }
            }
        });
        handles.push(handle);
    }

    drop(tx); // Close the sending end

    let mut results = vec![];
    for received in rx {
        println!(
            "{} -> {} ({} ms)",
            received.url,
            match &received.action_status {
                Ok(code) => code.to_string(),
                Err(e) => e.to_string(),
            },
            received.response_time.as_millis()
        );
        results.push(received);
    }

    for h in handles {
        let _ = h.join();
    }

    if let Err(e) = write_json_report(&results) {
        eprintln!("Failed to write status.json: {}", e);
    }
}
