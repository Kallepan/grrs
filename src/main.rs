use grrs::{error, flags::parser::get_args};

fn find_matches(content: String, pattern: &str, mut writer: impl std::io::Write) {
    for (_, line) in content.lines().enumerate() {
        if !line.contains(pattern) {
            continue;
        }

        match writeln!(writer, "{}", line) {
            Ok(_) => {}
            Err(err) => {
                error!("Could not write to output: {}", err);
                std::process::exit(1);
            }
        }
    }
}

fn main() {
    let args = get_args();

    let content = match std::fs::read_to_string(&args.path) {
        Ok(content) => content,
        Err(err) => {
            error!("Could not read file: {}", err);
            std::process::exit(1);
        }
    };

    find_matches(content, &args.pattern, &mut std::io::stdout());
}

#[test]
fn find_a_match() {
    let mut result = Vec::new();
    find_matches(
        String::from("lorem ipsum\ndolor sit amet"),
        "lorem",
        &mut result,
    );
    assert_eq!(result, b"lorem ipsum\n");
}
