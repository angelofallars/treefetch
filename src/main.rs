use clap::{app_from_crate, arg};
mod colors;
mod fields;

// Simple system fetch tool written in Rust.
fn main() {
    let matches = app_from_crate!()
        .arg(
            arg!([TREE])
                .help("Which tree to display.")
                .possible_values(["normal", "xmas", "bonsai"])
                .default_value("normal"),
        )
        .get_matches();

    let mut is_christmas: bool = false;

    macro_rules! with_color {
        ( $ascii: expr, $( $color: ident ),* ) => {
            format!(
                $ascii,
                $($color = colors::$color,)*
            ).lines().map( |s| s.trim().to_string() ).collect()
        }
    }

    let mut ascii_tree: Vec<String> = match matches.value_of("TREE").unwrap() {
        "normal" => {
            with_color!(
                "{green}     /\\*\\       {reset}
                {green}    /\\O\\*\\      {reset}
                {green}   /*/\\/\\/\\     {reset}
                {green}  /\\O\\/\\*\\/\\    {reset}
                {green} /\\*\\/\\*\\/\\/\\   {reset}
                {green} |O\\/\\/*/\\/O|   {reset}
                {yellow}      ||        {reset}
                {yellow}      ||        {reset}
                ",
                green,
                yellow,
                reset
            )
        }
        "bonsai" => {
            with_color!(
                "{green} {bold}             &               {reset}
                {green}          && & &&             {reset}
                {green}         &{yellow}_& & _/{green}&            {reset}
                {yellow}{bold}           /~\\                {reset}
                {green} &  & &{yellow}     /|                {reset}
                {green} & {yellow}{bold}_&{reset}{green}&{yellow}   _\\_/|   {green}             {reset}
                {green}&& {yellow}{bold}&{reset}{green}&&{yellow}_/    |\\     {green} && &      {reset}
                {green}  &&{yellow}_|/{green}{bold} &{reset}{yellow}  \\//~\\{green}{bold}   &&{reset}{yellow} &&{green}&  {reset}
                {yellow}            |/\\__/{green}& &{yellow}_/_{green}&&  {reset}
                {gray}        {bold}:{reset}{green}____{yellow}./~\\.{green}____{gray}{bold}:         {reset}
                {gray}{bold}         \\___________/         {reset}
                {gray}{bold}          (_)     (_)            {reset}
                ",
                gray, green, yellow, reset, bold
            )
        }

        "xmas" => {
            is_christmas = true;
            with_color!(
                "{bright_yellow}{bold}      ★         {reset}
                {green}     /\\{red}{bold}o{green}\\       {reset}
                {green}    /\\{red}{bold}o{green}\\*\\      {reset}
                {green}   /{red}{bold}o{green}/\\/\\{blue}{bold}o{green}\\     {reset}
                {green}  /\\O\\/\\{red}{bold}o{green}\\/{red}{bold}o{green}    {reset}
                {green} /{blue}{bold}o{green}*{red}{bold}o{green}/{blue}{bold}o{green}*\\/{red}{bold}o{green}/\\   {reset}
                {green} |O\\/\\/*/{red}{bold}o{green}/O|   {reset}
                {yellow}      ||        {reset}
                {yellow}      ||        {reset}
                ",
                red, green, blue, yellow, bright_yellow, bold, reset
            )
        }

        _ => unreachable!(),
    };

    let mut data_list: Vec<String> = Vec::new();

    // Hostname
    if let Ok(value) = fields::get_user_host_name(is_christmas) {
        data_list.push(value.0);
        data_list.push(value.1);
    };
    
    // Distro name

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

    //  Join lines
    let join = |mut left: Vec<String>, mut right: Vec<String>| -> Vec<String> {
        let mut i = 0;
        right.reverse();
        while let Some(mut right_item) = right.pop() {
            if is_christmas {
                right_item = right_item.replace("▪", &format!("{}▪{}", colors::red, colors::green))
            }
            left[i] = format!("{}{}", left[i], right_item);
            i += 1
        }
        left
    };
    
    let data_list = join(ascii_tree, data_list);

    for item in data_list.iter() {
        println!("{}", item);
    }
}
