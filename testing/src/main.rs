use std::fs::File;
use std::io::Write;
use std::process::Command;

struct LogSummary {
    info_count: u32,
    warn_count: u32,
    error_count: u32,
}

impl LogSummary {
    fn new() -> LogSummary {
        LogSummary {
            info_count: 0,
            warn_count: 0,
            error_count: 0,
        }
    }

    fn process_log(&mut self, log: &str) {
        if log.contains("INFO") {
            self.info_count += 1;
        } else if log.contains("WARN") {
            self.warn_count += 1;
        } else if log.contains("ERROR") {
            self.error_count += 1;
        }
    }

    fn save_to_file(&self) {
        let mut file = File::create("log_summary.txt").unwrap();
        writeln!(file, "INFO: {}", self.info_count).unwrap();
        writeln!(file, "WARN: {}", self.warn_count).unwrap();
        writeln!(file, "ERROR: {}", self.error_count).unwrap();
    }

    fn execute_python_script(&self) {
        Command::new("python3")
            .arg("generate_dashboard.py")
            .status()
            .unwrap();
    }
}

fn main() {
    let logs = [
        "INFO: Operation successful",
        "ERROR: Failed to connect",
        "WARN: Low battery",
        "INFO: Data synced",
        "ERROR: Timeout occurred",
    ];

    let mut summary = LogSummary::new();
    for log in logs.iter() {
        summary.process_log(log);
    }
    summary.save_to_file();
    summary.execute_python_script();
}