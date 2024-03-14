use clap::{App, Arg};
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{BufReader, Write};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;

// Struct representing a single monitor
#[derive(Debug, Deserialize, Serialize, Clone)]
struct Monitor {
    name: String,
    script: Option<String>,
    result: Option<Result>,
    code: String,
}

// Struct representing the 'Result' object
#[derive(Debug, Deserialize, Serialize, Clone)]
struct Result {
    value: i32,
    processed_at: DateTime<Utc>,
}

// Struct representing the JSON data structure
#[derive(Debug, Deserialize, Serialize, Clone)]
struct MonitorData {
    monitors: Vec<Monitor>,
}

fn update_monitors(monitor_data: Arc<Mutex<MonitorData>>) {
    loop {
        let mut data = monitor_data.lock().unwrap();
        for monitor in &mut data.monitors {
            // Generate random value
            let random_value = rand::random::<i32>();

            // Get current timestamp
            let current_timestamp = Utc::now();

            // Create Result instance with random value and current timestamp
            let result = Result {
                value: random_value,
                processed_at: current_timestamp,
            };

            // Update the result field of the monitor
            monitor.result = Some(result);
        }

        // Sleep for 30 seconds
        std::thread::sleep(std::time::Duration::from_secs(30));
    }
}

fn store_monitors(monitor_data: Arc<Mutex<MonitorData>>) {
    loop {
        let data = monitor_data.lock().unwrap().clone();
        let current_timestamp = Utc::now().format("%Y-%m-%d_%H-%M").to_string();
        let output_file_path = PathBuf::from(format!("{}_{}_monitors.json", current_timestamp, "current"));

        // Convert MonitorData to JSON
        let json_data = serde_json::to_string_pretty(&data).expect("Failed to convert to JSON");

        // Write the JSON data to the output file
        let mut output_file = File::create(&output_file_path).expect("Failed to create output file");
        output_file
            .write_all(json_data.as_bytes())
            .expect("Failed to write to output file");

        // Sleep for 1 minute
        std::thread::sleep(std::time::Duration::from_secs(60));
    }
}

fn process_monitors(monitor_data: Arc<Mutex<MonitorData>>) {
    let update_monitor_data = Arc::clone(&monitor_data);
    let store_monitor_data = Arc::clone(&monitor_data);

    let update_thread = thread::spawn(move || update_monitors(update_monitor_data));
    let store_thread = thread::spawn(move || store_monitors(store_monitor_data));

    let start_time = Instant::now();
    while start_time.elapsed() < Duration::from_secs(5 * 60) {
        // Wait for 5 minutes
    }

    // Terminate the threads
    update_thread.thread().unpark();
    store_thread.thread().unpark();
}






fn main() {
    // Define command-line arguments
    let matches = App::new("process_monitor")
        .arg(
            Arg::with_name("monitorFile")
                .long("monitorFile")
                .value_name("FILE")
                .required(true)
                .help("Path to the monitors.json file"),
        )
        .arg(
            Arg::with_name("outputFile")
                .long("outputFile")
                .value_name("FILE")
                .required(true)
                .help("Path to the output JSON file"),
        )
        .get_matches();

    // Extract the value of the "monitorFile" argument
    let monitor_file = matches.value_of_os("monitorFile").unwrap();
    let monitor_file_path = PathBuf::from(monitor_file);

    // Open and read the JSON file
    let file = File::open(&monitor_file_path).expect("Failed to open file");
    let reader = BufReader::new(file);

    // Parse JSON data
    let mut monitor_data: MonitorData =
        serde_json::from_reader(reader).expect("Error parsing JSON data");
         // Process and print monitor data
    for monitor in &mut monitor_data.monitors {
        // Generate random value
        let random_value = rand::random::<i32>();

        // Get current timestamp
        let current_timestamp = Utc::now();

        // Create Result instance with random value and current timestamp
        let result = Result {
            value: random_value,
            processed_at: current_timestamp,
        };

        // Update the result field of the monitor
        monitor.result = Some(result);
    }

    // Extract the value of the "outputFile" argument
    let output_file = matches.value_of_os("outputFile").unwrap();
    let output_file_path = PathBuf::from(output_file);

    // Convert MonitorData to JSON
    let json_data = serde_json::to_string_pretty(&monitor_data).expect("Failed to convert to JSON");

    // Write the JSON data to the output file
    let mut output_file = File::create(&output_file_path).expect("Failed to create output file");
    output_file
        .write_all(json_data.as_bytes())
        .expect("Failed to write to output file");

        let monitor_data = Arc::new(Mutex::new(monitor_data));
        process_monitors(monitor_data);
}
