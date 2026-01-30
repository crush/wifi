mod monitor;
mod network;

fn help() {
    println!("wifi              status (ip or ssid)");
    println!("wifi name         network name (slow)");
    println!("wifi list         select network");
    println!("wifi pass [name]  show password");
    println!("wifi signal       signal strength");
    println!("wifi speed        speed test");
    println!("wifi on/off       toggle wifi");
    println!("wifi <name>       connect");
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();

    match args.first().map(|s| s.as_str()) {
        None => network::current(),
        Some("name") => network::name(),
        Some("list") => network::list(),
        Some("pass") => network::pass(args.get(1).map(|s| s.as_str())),
        Some("signal") => monitor::signal(),
        Some("speed") => monitor::speed(),
        Some("off") => network::off(),
        Some("on") => network::on(),
        Some("-h" | "--help") => help(),
        Some(name) => network::connect(name),
    }
}
