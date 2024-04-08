use grrs::{error, flags::parser::get_args};

fn main() {
    let args = get_args();

    let content = match std::fs::read_to_string(&args.path) {
        Ok(content) => content,
        Err(err) => {
            error!("Could not read file: {}", err);
            std::process::exit(1);
        }
    };

    for (i, line) in content.lines().enumerate() {
        if line.contains(&args.pattern) {
            println!("{}\t{}", i, line);
        }
    }
}
