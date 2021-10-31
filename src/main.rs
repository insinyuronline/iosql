use std::process;

fn main() {
    if let Err(e) = iosql::run() {
        eprintln!("Application error: {}", e);

        process::exit(1)
    }
}
