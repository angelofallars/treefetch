use std::process::Command;
use regex::{Regex, Captures};
mod colors;

// Simple system fetch tool written in Rust.
fn main() {
    // Fetch the system data with command calls
    let username = run_command("whoami", vec!());
    let hostname = run_command("cat", vec!("/etc/hostname"));
    let uptime = run_command("uptime", vec!());
    let kernel = run_command("uname", vec!("-mrs"));
    let memory = run_command("free", vec!("-m"));
    let distro_data = run_command("/bin/sh", vec!("-c",
                                                  "cat /etc/*-release",
                                                  ));
    let shell = run_command("/bin/sh", vec!("-c",
                                            "echo $SHELL"));

    // Parse the distro name
    let re_distro = match_regex(&distro_data,
                                r#"(?x)
                                DISTRIB_DESCRIPTION=
                                (?P<distro_name>[^\n]+)\n
                                "#.to_string());
    let distro_name = re_distro.name("distro_name").unwrap().as_str();

    // Parse shell
    let re_shell = match_regex(&shell,
                               r#"(?x)
                               (?P<shell_name>[^/]+)$
                               "#.to_string());
    let shell = re_shell.name("shell_name").unwrap().as_str();

    // Parse the uptime in hours/minutes
    let re_uptime = match_regex(&uptime,
                                r#"(?x)
                                (?P<hours>\d+)
                                :
                                (?P<minutes>\d+)
                                ,
                                "#.to_string());
    let hours = re_uptime.name("hours").unwrap().as_str();
    let minutes = re_uptime.name("minutes").unwrap().as_str();

    // Parse the kernel
    let re_kernel = match_regex(&kernel,
                                r#"(?x)
                                (?P<kernel_name>\S+)
                                \s+
                                (?P<kernel_version>\S+)"#.to_string());
    let kernel = re_kernel.name("kernel_version").unwrap().as_str();

    // Parse total/used RAM
    let re_memory = match_regex(&memory,
                                r#"(?x)
                                Mem:
                                \s+
                                (?P<total>\d+)
                                \s+
                                (?P<used>\d+)
                                "#.to_string());
    let total_mem = re_memory.name("total").unwrap().as_str();
    let used_mem = re_memory.name("used").unwrap().as_str();


    // Print the system data
    println!("{color}{bold}{user}{reset}@{color}{bold}{host}{reset}",
             user = username,
             host = hostname,
             color = colors::green,
             bold = colors::bold,
             reset = colors::reset,
             );

    print_data("os", &distro_name);

    print_data("kernel", &kernel);

    print_data("shell", &shell);

    print_data(
        "uptime",
        &format!("{hours}h {minutes}m",
                 hours = hours,
                 minutes = minutes)
        );

    print_data(
        "memory",
        &format!("{used}m / {total}m",
                 used = used_mem,
                 total = total_mem)
        );
}

fn print_data(key: &str, value: &str) {
    println!("{color}{bold}{key:6}{reset} {value}",
            key = key,
            value = value,
            color = colors::green,
            bold = colors::bold,
            reset = colors::reset,
            );
}

// Search with Regex in a string and return all of the matches
fn match_regex(search_str: &String, regex: String) -> Captures {
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
