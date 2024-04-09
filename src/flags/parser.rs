use crate::error;

pub struct Cli {
    // the pattern to search for
    pub pattern: String,
    // the path to the file to search
    pub path: std::path::PathBuf,
}

pub fn get_args() -> Cli {
    // check if special first argument is provided
    // i.e.:
    // --help or -h
    // --version or -v
    match std::env::args().nth(1) {
        Some(arg) => match arg.as_str() {
            "--help" | "-h" => {
                println!("grrs");
                println!("Usage: grrs PATTERN PATH");
                println!();
                println!("Search for PATTERN in each FILE.");
                println!("Example: grrs foo /tmp/bar");
                std::process::exit(0);
            }
            "--version" | "-v" => {
                println!("grrs version 0.1.0");
                std::process::exit(0);
            }
            _ => {}
        },
        // continue if no special first argument is provided
        None => {}
    };

    let error_msg = "Error: The following required arguments were not provided:\n\
    \t<PATTERN>\n\
    \t<PATH>\n\
    USAGE:\n\
    \tgrrs <PATTERN> <PATH>\n\
    For more information try --help\n";

    let pattern = match std::env::args().nth(1) {
        Some(arg) => arg,
        None => {
            println!("{}", error_msg);
            error!("No pattern provided");
            std::process::exit(1);
        }
    };

    if pattern.is_empty() {
        println!("{}", error_msg);
        error!("Pattern is empty");
        std::process::exit(1);
    }

    let path = match std::env::args().nth(2) {
        Some(arg) => arg,
        None => {
            println!("{}", error_msg);
            error!("No path provided");
            std::process::exit(1);
        }
    };

    Cli {
        pattern,
        path: std::path::PathBuf::from(path),
    }
}
