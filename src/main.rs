mod config;
mod checker;

fn main() {
    let cfg = config::load("config.toml");
    for service in &cfg.service {
        println!("{}: checking...", service.name);
    }
    let up = checker::check_port("127.0.0.1:80");
    println!("nginx up: {}", up);
}
