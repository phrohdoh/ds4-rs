extern crate ds4;

fn main() {
    if let Err(e) = ds4::run() {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}