use std::env;
use std::fs::OpenOptions;
use std::io::{self, BufRead, BufReader, BufWriter, Write};

use dashmap::DashMap;

const HISTORY_NAME_FILE: &str = "crabis.aof";

// Returns true if args contains "AOF" or "aof"
pub fn is_aof_enabled() -> bool {
    env::args().any(|arg| {
        let upper = arg.to_uppercase();
        upper == "AOF"
    })
}

// Append a command line to the AOF file.
pub fn append_to_aof_file(line: &str) -> io::Result<()> {
    let file = OpenOptions::new().create(true).append(true).open(HISTORY_NAME_FILE)?;

    let mut writer = BufWriter::new(file);
    writer.write_all(line.as_bytes())?;
    writer.write_all(b"\r\n")?;
    writer.flush()
}

// Load all previously stored commands from the AOF file (Empty vector if the file does not exist yet or is empty).
pub fn load_history_from_file() -> io::Result<Vec<String>> {
    let file = match OpenOptions::new().read(true).open(HISTORY_NAME_FILE) {
        Ok(f) => f,
        Err(e) if e.kind() == io::ErrorKind::NotFound => {
            println!("AOF file not found, no commands to replay");
            return Ok(Vec::new());
        }
        Err(e) => return Err(e),
    };

    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;
    println!("Loaded {} commands from the AOF file", lines.len());
    Ok(lines)
}

// Replay the history into the in‑memory store.
pub fn replay_history(lines: &[String], store: &DashMap<String, String>) {
    for line in lines {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.is_empty() {
            continue;
        }

        match parts[0].to_uppercase().as_str() {
            "SET" if parts.len() >= 3 => {
                store.insert(parts[1].to_string(), parts[2].to_string());
            }
            _ => {
                println!("Unknown or unsupported command in history: {}, no action taken", line);
            }
        }
    }
    if lines.is_empty() {
        println!("No commands to replay");
    } else {
        println!("History replayed into the in‑memory store");
    }
}