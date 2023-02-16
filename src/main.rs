fn main() {
    let args: Vec<String> = std::env::args().collect();
    if let Err(e) = lib::parse_cli_args(args) {
        println!("{}{}", ansi_term::Colour::Red.bold().paint("Error: "), e);
    }
}
