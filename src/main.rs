mod config;
mod checker;
mod state;

use config::CheckType;
use state::ServiceStatus;

#[tokio::main]
async fn main() {
    let cfg = config::load("config.toml");
    let shared = state::init();
    loop {
        for service in &cfg.service {
            let up = match service.check {
                CheckType::Port => checker::check_port(&service.target),
                CheckType::Http => checker::check_http(&service.target).await,
                CheckType::Process => {
                    println!("{}: process check not implemented yet", service.name);
                    false
                },
            };
            let mut map = shared.lock().await;
            map.insert(service.name.clone(), ServiceStatus { up });

            println!("{}: {}", service.name, if up { "UP" } else { "DOWN" });
        }
        tokio::time::sleep(std::time::Duration::from_secs(15)).await;
    }
}
