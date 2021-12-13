use std::process::Command;
use regex::{Regex, Captures};
mod colors;

// Simple system fetch tool written in Rust.
fn main() {
    let ascii_tree = format!("{green}     /\\*\\      {reset}
{green}    /\\O\\*\\     {reset}
{green}   /*/\\/\\/\\    {reset}
{green}  /\\O\\/\\*\\/\\   {reset}
{green} /\\*\\/\\*\\/\\/\\  {reset}
{green} |O\\/\\/*/\\/O|  {reset}
{yellow}      ||       {reset}
{yellow}      ||       {reset}
",
    green = colors::green,
    yellow = colors::yellow,
    reset = colors::reset,
);

    let ascii_tree = split_by_newline(ascii_tree);

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
                                (?P<distro_name>[^\n"]+)\n
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
    let mut data_list: Vec<String> = Vec::new();

    data_list.push(format!("{color}{bold}{user}{reset}{bold}@{color}{host}{reset}",
             user = username,
             host = hostname,
             color = colors::green,
             bold = colors::bold,
             reset = colors::reset,
             ));

    data_list.push(format_data("os", &distro_name));

    data_list.push(format_data("kernel", &kernel));

    data_list.push(format_data("shell", &shell));

    data_list.push(format_data(
        "uptime",
        &format!("{hours}h {minutes}m",
                 hours = hours,
                 minutes = minutes)
        ));

    data_list.push(format_data(
        "memory",
        &format!("{used}m / {total}m",
                 used = used_mem,
                 total = total_mem)
        ));

    print_left_to_right(ascii_tree, data_list);
}

// Print two vectors of strings side to side
fn print_left_to_right(left: Vec<String>, right: Vec<String>) {
    let left_len = left.len();
    let right_len = right.len();
    let max_len = if left_len > right_len {left_len} else {right_len};

    for i in 0..max_len {
        if i < left_len {
            print!("{}", left[i].replace("\n", ""));
        }
        if i < right_len {
            print!("{}", right[i].trim());
        }

        // Print a newline
        println!("");
    }
}

fn split_by_newline(ascii_art: String) -> Vec<String> {
    let mut split: Vec<String> = Vec::new();
    let mut last_index = 0;

    let bytes = ascii_art.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b'\n' {
            split.push(ascii_art[last_index..i].to_string());
            last_index = i;
        }
    }

    split
}

fn format_data(key: &str, value: &str) -> String {
    format!("{color}{bold}{key:6}{reset} {value}",
            key = key,
            value = value,
            color = colors::green,
            bold = colors::bold,
            reset = colors::reset,
            )
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
