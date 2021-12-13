mod colors;
mod fields;

// Simple system fetch tool written in Rust.
fn main() {
    let ascii_tree = format!("{green}     /\\*\\       {reset}
{green}    /\\O\\*\\      {reset}
{green}   /*/\\/\\/\\     {reset}
{green}  /\\O\\/\\*\\/\\    {reset}
{green} /\\*\\/\\*\\/\\/\\   {reset}
{green} |O\\/\\/*/\\/O|   {reset}
{yellow}      ||        {reset}
{yellow}      ||        {reset}
",
    green = colors::green,
    yellow = colors::yellow,
    reset = colors::reset,
);
    let ascii_tree = split_by_newline(ascii_tree);

    let mut data_list: Vec<String> = Vec::new();

    match fields::get_user_host_name() {
        Ok(value) => {
            data_list.push(value.0);
            data_list.push(value.1);
        },
        Err(_) => {}
    };

    match fields::get_distro_name() {
        Ok(value) => data_list.push(value),
        Err(_) => {}
    };

    // Kernel name

    match fields::get_kernel() {
        Ok(value) => data_list.push(value),
        Err(_) => {}
    };

    // Shell

    match fields::get_shell() {
        Ok(value) => data_list.push(value),
        Err(_) => {}
    };

    // Uptime

    match fields::get_uptime() {
        Ok(value) => data_list.push(value),
        Err(_) => {}
    };

    // Memory

    match fields::get_memory() {
        Ok(value) => data_list.push(value),
        Err(_) => {}
    };

    print_left_to_right(ascii_tree, data_list);
}

// Print two vectors of strings side to side
fn print_left_to_right(left: Vec<String>, right: Vec<String>) {
    let left_len = left.len();
    let right_len = right.len();
    let max_len = if left_len > right_len {left_len} else {right_len};

    for i in 0..max_len {
        if i < left_len {
            print!("{}", left[i]);
        }
        if i < right_len {
            print!("{}", right[i]);
        }

        // Print a newline
        println!("");
    }
}

// Split a multi-line string into several ones separated by the newline
fn split_by_newline(ascii_art: String) -> Vec<String> {
    let mut split: Vec<String> = Vec::new();
    let mut last_index = 0;

    let bytes = ascii_art.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b'\n' {
            split.push(ascii_art[last_index..i].trim().to_string());
            last_index = i;
        }
    }

    split
}
