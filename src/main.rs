mod monitor;
mod network;
mod style;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();

    match args.first().map(|s| s.as_str()) {
        None => network::current(),
        Some("list") => network::list(),
        Some("pass") => network::pass(args.get(1).map(|s| s.as_str())),
        Some("signal") => monitor::signal(),
        Some("speed") => monitor::speed(),
        Some("off") => network::off(),
        Some("on") => network::on(),
        Some(name) => network::connect(name),
    }
}
