use std::fs::File;
use std::io::{BufRead, BufReader};
use std::sync::{Arc, Mutex, mpsc::{self, Sender, Receiver}};
use std::thread;
use std::time::{Duration, Instant, SystemTime};
use std::collections::HashMap;
use ureq;

const DEFAULT_TIMEOUT: u64 = 5;
const MAX_RETRIES: u8 = 3;

type Url = String;

enum MonitorMessage {
    CheckUrl(Url),
    Shutdown,
}

struct Config {
    worker_threads: usize,
    timeout: Duration,
    max_retries: u8,
}

impl Config {
    fn new(worker_threads: usize, timeout: Duration, max_retries: u8) -> Self {
        Config {
            worker_threads,
            timeout,
            max_retries,
        }
    }
}

struct WebsiteStatus {
    url: String,
    status: Result<u16, String>,
    response_time: Duration,
    timestamp: SystemTime,
}

fn check_website(url: &str, timeout: Duration, max_retries: u8) -> WebsiteStatus {
    let mut retries = 0;
    let mut last_error = None;
    let start = Instant::now();

    while retries < max_retries {
        let response = ureq::get(url).timeout(timeout).call();

        if let Ok(response) = response {
            let elapsed = start.elapsed();
            return WebsiteStatus {
                url: url.to_string(),
                status: Ok(response.status()),
                response_time: elapsed,
                timestamp: SystemTime::now(),
            };
        } else {
            last_error = Some(format!("{}", response.unwrap_err()));
            retries += 1;
        }
    }

    WebsiteStatus {
        url: url.to_string(),
        status: Err(last_error.unwrap_or("Unknown error".to_string())),
        response_time: Duration::ZERO,
        timestamp: SystemTime::now(),
    }
}

fn monitor_worker(receiver: Arc<Mutex<Receiver<MonitorMessage>>>, sender: Sender<WebsiteStatus>, timeout: Duration, max_retries: u8) {
    while let Ok(message) = receiver.lock().unwrap().recv() {
        match message {
            MonitorMessage::CheckUrl(url) => {
                let status = check_website(&url, timeout, max_retries);
                sender.send(status).unwrap();
            }
            MonitorMessage::Shutdown => {
                break;
            }
        }
    }
}

fn main() {
    // Open the file containing URLs
    let file = File::open("urls.txt").expect("Failed to open file urls.txt");
    let reader = BufReader::new(file);

    // Read URLs line by line into a vector
    let urls: Vec<String> = reader
        .lines()
        .map(|line| line.expect("Failed to read line"))
        .collect();

    if urls.is_empty() {
        eprintln!("No URLs found in the file.");
        return;
    }

    let config = Config::new(10, Duration::from_secs(DEFAULT_TIMEOUT), MAX_RETRIES);

    let (task_sender, task_receiver) = mpsc::channel();
    let (result_sender, result_receiver) = mpsc::channel();

    let task_receiver = std::sync::Arc::new(std::sync::Mutex::new(task_receiver));

    let mut workers = vec![];

    for _ in 0..config.worker_threads {
        let task_receiver = std::sync::Arc::clone(&task_receiver);
        let result_sender = result_sender.clone();

        let timeout = config.timeout;
        let max_retries = config.max_retries;

        let handle = thread::spawn(move || {
            monitor_worker(task_receiver, result_sender, timeout, max_retries);
        });

        workers.push(handle);
    }

    for url in urls {
        task_sender.send(MonitorMessage::CheckUrl(url)).unwrap();
    }

    drop(task_sender);

    for received in result_receiver {
        match received.status {
            Ok(status) => println!("{} - Status: {} - Time: {:?} - Timestamp: {:?}", received.url, status, received.response_time, received.timestamp),
            Err(err) => println!("{} - Failed: {} - Timestamp: {:?}", received.url, err, received.timestamp),
        }
    }

    for worker in workers {
        worker.join().expect("Worker thread panicked");
    }

    println!("All workers shut down gracefully.");
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::mpsc;

    #[test]
    fn test_check_website_success() {
        // Mock URL that always succeeds
        let mock_url = "https://httpstat.us/200";
        let timeout = Duration::from_secs(DEFAULT_TIMEOUT);
        let max_retries = MAX_RETRIES;

        let status = check_website(mock_url, timeout, max_retries);
        assert!(status.status.is_ok());
        assert_eq!(status.status.unwrap(), 200);
        assert!(status.response_time > Duration::ZERO);
    }

    #[test]
    fn test_check_website_failure() {
        // Mock URL that will fail
        let mock_url = "https://invalid.url";
        let timeout = Duration::from_secs(DEFAULT_TIMEOUT);
        let max_retries = MAX_RETRIES;

        let status = check_website(mock_url, timeout, max_retries);
        assert!(status.status.is_err());
        assert_eq!(status.response_time, Duration::ZERO);
    }

    #[test]
    fn test_monitor_worker() {
        let (task_sender, task_receiver) = mpsc::channel();
        let (result_sender, result_receiver) = mpsc::channel();
        let task_receiver = Arc::new(Mutex::new(task_receiver));

        let timeout = Duration::from_secs(DEFAULT_TIMEOUT);
        let max_retries = MAX_RETRIES;

        // Start a worker thread
        let worker_thread = thread::spawn({
            let task_receiver = Arc::clone(&task_receiver);
            move || monitor_worker(task_receiver, result_sender, timeout, max_retries)
        });

        // Send a mock task to the worker
        task_sender.send(MonitorMessage::CheckUrl("https://httpstat.us/200".to_string())).unwrap();

        // Drop the sender to ensure the worker thread finishes
        drop(task_sender);

        // Collect the result
        let result = result_receiver.recv().unwrap();
        assert!(result.status.is_ok());
        assert_eq!(result.status.unwrap(), 200);

        // Ensure the worker thread shuts down cleanly
        worker_thread.join().unwrap();
    }

    #[test]
    fn test_graceful_shutdown() {
        let (task_sender, task_receiver) = mpsc::channel();
        let (result_sender, _result_receiver) = mpsc::channel();
        let task_receiver = Arc::new(Mutex::new(task_receiver));

        let timeout = Duration::from_secs(DEFAULT_TIMEOUT);
        let max_retries = MAX_RETRIES;

        let worker_thread = thread::spawn({
            let task_receiver = Arc::clone(&task_receiver);
            move || monitor_worker(task_receiver, result_sender, timeout, max_retries)
        });

        // Send shutdown signal
        task_sender.send(MonitorMessage::Shutdown).unwrap();

        // Ensure the worker thread shuts down cleanly
        worker_thread.join().unwrap();
    }
}
