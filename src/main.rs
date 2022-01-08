use std::env;
mod colors;
mod fields;

// Simple system fetch tool written in Rust.
fn main() {
    let mut ascii_tree = format!("{green}     /\\*\\       {reset}
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

    // Christmas tree if passed with -xmas argument
    let args: Vec<String> = env::args().collect();
    let mut is_christmas = false;

    if args.len() >= 2 && args[1] == "-xmas" {
            ascii_tree = format!("{bright_yellow}{bold}      ★         {reset}
{green}     /\\{red}{bold}o{green}\\       {reset}
{green}    /\\{red}{bold}o{green}\\*\\      {reset}
{green}   /{red}{bold}o{green}/\\/\\{blue}{bold}o{green}\\     {reset}
{green}  /\\O\\/\\{red}{bold}o{green}\\/{red}{bold}o{green}    {reset}
{green} /{blue}{bold}o{green}*{red}{bold}o{green}/{blue}{bold}o{green}*\\/{red}{bold}o{green}/\\   {reset}
{green} |O\\/\\/*/{red}{bold}o{green}/O|   {reset}
{yellow}      ||        {reset}
    ",
            red = colors::red,
            green = colors::green,
            blue = colors::blue,
            yellow = colors::yellow,
            bright_yellow = "\x1b[93m",
            bold = colors::bold,
            reset = colors::reset,
            );
            is_christmas = true;
    }

    let ascii_tree = split_by_newline(ascii_tree);

    let mut data_list: Vec<String> = Vec::new();

    if let Ok(value) = fields::get_user_host_name(is_christmas) {
            data_list.push(value.0);
            data_list.push(value.1);
    };


    if let Ok(value) = fields::get_distro_name() {
        data_list.push(value);
    };

    // Kernel name

    if let Ok(value) = fields::get_kernel() {
        data_list.push(value);
    };

    // Shell

    if let Ok(value) = fields::get_shell() {
        data_list.push(value);
    };

    // Uptime

    if let Ok(value) = fields::get_uptime() {
        data_list.push(value);
    };

    // Memory

    if let Ok(value) = fields::get_memory() {
        data_list.push(value);
    };

    print_left_to_right(ascii_tree, data_list, is_christmas);
}

// Print two vectors of strings side to side
fn print_left_to_right(left: Vec<String>, right: Vec<String>,
                       is_christmas: bool) {
    let left_len = left.len();
    let right_len = right.len();
    let max_len = if left_len > right_len {left_len} else {right_len};

    for i in 0..max_len {
        if i < left_len {
            print!("{}", left[i]);
        }
        if i < right_len {

            // Red square if Christmas mode
            if is_christmas {
                print!("{}", right[i]
                       .replace("▪",
                                &format!("{}▪{}",
                                         colors::red,
                                         colors::green)));
            } else {
                print!("{}", right[i]);
            }
        }

        // Print a newline
        println!();
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
