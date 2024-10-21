use std::env;
use std::fs::File;
use std::io::{BufReader, Read, Write};
use std::fs::OpenOptions;
use serde_json;
use base64;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: {} <command>", args[0]);
        return;
    }

    let command = &args[1];

    run_command(command, args.clone());
}

fn run_command(command: &str, args: Vec<String>) {
    match command {
        "help" => help_command(args),
        "add" => add_command(args),
        "read-db" => read_db_command(),
        "search" => search_command(args),
        _ => println!("Unknown command {}", command),
    }
}

fn help_command(args: Vec<String>) {
    if args.len() < 3 {
        println!("Available commands:\n");

        // println!("add - Adds new entry to local database");
        println!("{} add <identifier> <entry> - Adds new entry to local database", args[0]);

        println!("read-db - Reads all entries from local database");

        // println!("search - Searches for an entry in the local database");
        println!("{} search <entry> - Searches for an entry in the local database", args[0]);

        println!("help - Prints this help message");
        println!("{} help <command> - Prints help message for the given command", args[0]);
        return;
    }

    let command = &args[2];

    match command.as_str() {
        "help" => println!("Prints a help message"),
        "add" => println!("Adds new entry to local database"),
        "read-db" => println!("Reads all entries from local database"),
        "search" => println!("Searches for an entry in the local database"),
        _ => println!("Unknown command {}", command),
    }

}

fn add_command(args: Vec<String>) {
    if args.len() < 4 {
        println!("Usage: {} add <identifier> <entry>", args[0]);
        return;
    }

    let identifier = &args[2];
    let entry = &args[3];

    if File::open("config.json").is_err() {
        println!("Configuration file not found");
        println!("Creating configuration file");
        
        let mut config = File::create("config.json").expect("Failed to create configuration file");
        config.write(b"{}").expect("Failed to write configuration file");
    }

    let mut config: serde_json::Value = serde_json::from_reader(File::open("config.json").expect("Failed to open configuration file")).expect("Failed to parse configuration file");

    if config["database"].is_null() {
        println!("Database path not found in configuration file");
        println!("Creating database.fasta file");

        File::create("database.fasta").expect("Failed to create database file");

        config["database"] = "database.fasta".into();
        println!("Writing configuration file");
        serde_json::to_writer_pretty(File::create("config.json").expect("Failed to create configuration file"), &config).expect("Failed to write configuration file");
    }

    if File::open(config["database"].as_str().expect("Failed to parse database path")).is_err() {
        println!("Database file not found");
        println!("Creating database file");

        File::create(config["database"].as_str().expect("Failed to parse database path")).expect("Failed to create database file");
    }

    let input = format!("{}:{}", identifier, entry);
    let mut encoded = base64::encode(input.as_bytes());
    encoded.push_str("\n");

    let mut database = OpenOptions::new().append(true).open(config["database"].as_str().expect("Failed to parse database path")).expect("Failed to open database file");
    database.write(encoded.as_bytes()).expect("Failed to write to database file");


    println!("Adding entry with identifier {} and entry {}", identifier, entry);
}

fn read_db_command() {
    let config: serde_json::Value = serde_json::from_reader(File::open("config.json").expect("Failed to open configuration file")).expect("Failed to parse configuration file");

    let database = File::open(config["database"].as_str().expect("Failed to parse database path")).expect("Failed to open database file");
    let mut reader = BufReader::new(database);

    let mut buffer = String::new();
    reader.read_to_string(&mut buffer).expect("Failed to read database file");

    let entries: Vec<&str> = buffer.split("\n").collect();
    println!("Database entries:");
    for entry in entries {
        if entry.is_empty() {
            continue;
        }

        let decoded = base64::decode(entry).expect("Failed to decode entry");
        let decoded = String::from_utf8(decoded).expect("Failed to convert decoded entry to string");

        let parts: Vec<&str> = decoded.split(":").collect();
        println!("Identifier: {}, Entry: {}", parts[0], parts[1]);
    }

}

fn search_command(args: Vec<String>) {
    if args.len() < 3 {
        println!("Usage: {} search <entry>", args[0]);
        return;
    }

    let identifier = &args[2];

    let config: serde_json::Value = serde_json::from_reader(File::open("config.json").expect("Failed to open configuration file")).expect("Failed to parse configuration file");

    let database = File::open(config["database"].as_str().expect("Failed to parse database path")).expect("Failed to open database file");
    let mut reader = BufReader::new(database);

    let mut buffer = String::new();
    reader.read_to_string(&mut buffer).expect("Failed to read database file");

    let entries: Vec<&str> = buffer.split("\n").collect();
    // println!("Entries: {:?}", entries);
    for entry in entries {
        if entry.is_empty() {
            continue;
        }

        let decoded = base64::decode(entry).expect("Failed to decode entry");
        let decoded = String::from_utf8(decoded).expect("Failed to convert decoded entry to string");

        let parts: Vec<&str> = decoded.split(":").collect();
        if parts[1] == identifier {
            // println!("Entry found: {}", parts[1]);

            println!("Entry found:");
            println!("Identifier: {}\nEntry: {}", parts[0], parts[1]);
            
            return;
        }
    }

    println!("Entry not found for identifier {}", identifier);
}