mod ls;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    // Default to current directory if no path specified
    let dir_path = if args.len() > 1 {
        &args[1]
    } else {
        "."
    };
    
    // Parse options
    let mut options = ls::LsOptions::default();
    
    for arg in &args[1..] {
        if arg.starts_with('-') {
            for flag in arg.chars().skip(1) {
                match flag {
                    'a' => options.show_hidden = true,
                    'l' => options.long_format = true,
                    _ => (),
                }
            }
        }
    }
    
    // Run the ls command
    if let Err(e) = ls::run(dir_path, &options) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
