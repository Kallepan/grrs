use grrs::{error, flags::parser::get_args};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");

    let args = get_args();

    let content = std::fs::read_to_string(&args.path).expect("could not read file");
    find_matches(content, &args.pattern, &mut std::io::stdout());

    Ok(())
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
