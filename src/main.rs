mod config;
mod checker;

use config::CheckType;

#[tokio::main]
async fn main() {
    let cfg = config::load("config.toml");

    for service in &cfg.service {
        let up = match service.check {
            CheckType::Port => checker::check_port(&service.target),
            CheckType::Http => checker::check_http(&service.target).await,
            CheckType::Process => {
                println!("{}: process check not implemented yet", service.name);
                false
            },
        };
        println!("{}: {}", service.name, if up { "UP" } else { "DOWN" });
    }
}
