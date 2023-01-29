fn main() {
    if let Err(e) = rust_headr::get_args().and_then(rust_headr::run) {
        eprintln!("{}",e);
        std::process::exit(1);
    }
}
