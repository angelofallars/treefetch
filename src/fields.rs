use std::io::Read;
use std::env;
use std::fs;
use regex::{Regex, Captures};
use crate::colors;

fn format_data(key: &str, value: &str) -> String {
    format!("{color}▪{bold} {key:7}{reset} {value}",
            key = key,
            value = value,
            color = colors::green,
            bold = colors::bold,
            reset = colors::reset,
            )
}

// Search with Regex in a string and return all of the matches
fn match_regex(search_str: &String, regex: String) -> Option<Captures> {
    let re = Regex::new(&regex).unwrap();

    re.captures(&search_str)
}

pub fn get_user_host_name() -> Result<(String, String), String> {
    // Username
    let username_env = env::var_os("USER");
    let username: String;

    if username_env.is_some() {
        username = username_env.unwrap().into_string().unwrap();
    } else {
        username = String::new();
    }

    // Hostname
    let hostname_file = fs::File::open("/etc/hostname");

    if hostname_file.is_err() {
        return Err("error".to_string());
    }

    let mut hostname_file = hostname_file.unwrap();
    let mut hostname = String::new();

    let result = hostname_file.read_to_string(&mut hostname);

    if result.is_err() {
        return Err("error".to_string());
    }

    // Combine username and hostname into a formatted string

    let user_host_name = format!("{color}{bold}{user}{reset}
                                 {bold}@{color}{host}{reset}",
                                 user = username,
                                 host = hostname,
                                 color = colors::green,
                                 bold = colors::bold,
                                 reset = colors::reset,
                                 ).replace(" ", "").replace("\n", "");

    // Separator
    // format: username length + @ (1) + hostname length

    let user_host_name_len = username.len() + 1 + hostname.len();
    let mut separator = String::new();

    separator += colors::yellow;
    for _i in 0..(user_host_name_len) {
        separator += "━";
    }
    separator += colors::reset;

    return Ok((user_host_name, separator));
}

pub fn get_distro_name() -> Result<String, String> {
    // First get the lsb-release file
    let lsb_release = fs::File::open("/etc/lsb-release");
    let mut buffer = String::new();

    // Check if lsb_release exists
    if lsb_release.is_ok() {
        // Read lsb_release into buffer
        let mut lsb_release = lsb_release.unwrap();
        let result = lsb_release.read_to_string(&mut buffer);

        if result.is_err() { return Err("error".to_string()); }

        // Match regex in buffer
        let re_lsb = match_regex(&buffer,
                                 r#"(?x)
                                 DISTRIB_DESCRIPTION=
                                 "?   # Quotes if description is multiple words
                                 (?P<distro_name>[^\n"]+)
                                 "?   # Ditto
                                 \n
                                 "#.to_string());

        // Check if regex matches
        if re_lsb.is_some() {
            let re_lsb = re_lsb.unwrap();

            let distro_name = re_lsb.name("distro_name")
                .unwrap()
                .as_str();
            return Ok(format_data("os", &distro_name));
        }
    }

    // If no lsb-release then fetch os-release
    let os_release = fs::File::open("/etc/os-release");

    if os_release.is_err() {
        return Err("Error".to_string());
    }

    let mut os_release = os_release.unwrap();
    let result = os_release.read_to_string(&mut buffer);

    if result.is_err() { return Err("error".to_string()); }

    let re_os = match_regex(&buffer,
                            r#"(?x)
                            PRETTY_NAME=
                            "?   # Quotes if description is multiple words
                            (?P<distro_name>[^\n"]+)
                            "?   # Ditto
                            \n
                            "#.to_string());

    if re_os.is_some() {
        let re_os = re_os.unwrap();

        let distro_name = re_os.name("distro_name")
            .unwrap()
            .as_str();
        return Ok(format_data("os", &distro_name));
    }

    return Err("error".to_string());
}

pub fn get_kernel() -> Result<String, String> {
    let kernel_file = fs::File::open("/proc/version");

    if kernel_file.is_err() {
        return Err("Error".to_string());
    }

    let mut kernel_file = kernel_file.unwrap();
    let mut kernel = String::new();

    let result = kernel_file.read_to_string(&mut kernel);

    if result.is_err() {
        return Err("error".to_string());
    }

    let re_kernel = match_regex(&kernel,
                                r#"(?x)
                                Linux\sversion\s
                                (?P<kernel_version>\S+)"#.to_string());

    if re_kernel.is_none() {
        return Err("Error".to_string());
    } else {
        let re_kernel = re_kernel.unwrap();

        let kernel = re_kernel.name("kernel_version").unwrap().as_str();
        return Ok(format_data("kernel", &kernel));
    }
}

pub fn get_shell() -> Result<String, String> {
    let shell_env = env::var_os("SHELL");

    if shell_env.is_none() {
        return Err("Error".to_string());
    }

    let shell = shell_env.unwrap().into_string().unwrap();

    let re_shell = match_regex(&shell,
                               r#"(?x)
                               (?P<shell_name>[^/]+)$
                               "#.to_string());

    if re_shell.is_none() {
        return Err("Error".to_string());
    }

    let re_shell = re_shell.unwrap();

    let shell = re_shell.name("shell_name").unwrap().as_str();
    return Ok(format_data("shell", &shell));
}