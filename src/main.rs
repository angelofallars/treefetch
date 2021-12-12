use std::process::Command;
use regex::{Regex, Captures};

// Color/formatting codes for output
const _RED: &str = "\x1b[31m";
const _GREEN: &str = "\x1b[32m";
const _YELLOW: &str = "\x1b[33m";
const _BLUE: &str = "\x1b[34m";
const _CYAN: &str = "\x1b[36m";
const _MAGENTA: &str = "\x1b[35m";
const _BOLD: &str = "\x1b[1m";
const _RESET: &str = "\x1b[0m";

// Simple system fetch tool written in Rust.
fn main() {
    // Fetch the system data with command calls
    let username = run_command("whoami", vec!());
    let hostname = run_command("cat", vec!("/etc/hostname"));

    // Fetch the uptime in hours/minutes
    let uptime = run_command("uptime", vec!());

    let re_uptime = get_regex_capture(&uptime,
                                      r"(?x)
                                      (?P<hours>\d+)
                                      :
                                      (?P<minutes>\d+)
                                      ,
                                      ".to_string());
    let hours = re_uptime.name("hours").unwrap().as_str();
    let minutes = re_uptime.name("minutes").unwrap().as_str();

    // Fetch total/used RAM
    let memory = run_command("free", vec!("-m"));

    let re_memory = get_regex_capture(&memory,
                                      r"(?x)
                                      Mem:
                                      \s+
                                      (?P<total>\d+)
                                      \s+
                                      (?P<used>\d+)
                                      ".to_string());
    let total_mem = re_memory.name("total").unwrap().as_str();
    let used_mem = re_memory.name("used").unwrap().as_str();


    // Print the system data
    // Username + Hostname
    println!("{color}{bold}{user}{reset}@{color}{bold}{host}{reset}",
             user = username,
             host = hostname,
             color = _GREEN,
             bold = _BOLD,
             reset = _RESET,
             );
    // Uptime
    println!("{}", format_data(
            "uptime",
            &format!("{hours}h / {minutes}m",
                     hours = hours,
                     minutes = minutes)
            ));
    // Memory
    println!("{}", format_data(
            "memory",
            &format!("{total}m / {used}m",
                     total = total_mem,
                     used = used_mem)
            ));
}

fn format_data(key: &str, value: &str) -> String {
    format!("{color}{bold}{key}{reset} {value}",
            key = key,
            value = value,
            color = _GREEN,
            bold = _BOLD,
            reset = _RESET,
            ).to_string()
}

// Search with Regex in a string and return all of the matches
fn get_regex_capture(search_str: &String, regex: String) -> Captures {
    let re = Regex::new(&regex).unwrap();

    re.captures(&search_str).unwrap()
}

// Run a command and return the output
fn run_command(command: &str, args: Vec<&str>) -> String {
    // Initialize the process
    let mut command = Command::new(command);
    // Add the arguments
    command.args(args);

    // Run the command
    let output = command
                 .output()
                 .expect("failed to execute process");

    // Return the output (stdout)
    let stdout = String::from_utf8(output.stdout)
                 .unwrap();
    stdout.trim().to_string()
}
