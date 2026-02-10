use crate::config::root::configuration::Configuration;

mod config;

fn main() {
    let config: Configuration =
        match crate::config::root::configuration::Configuration::from_env() {
            Ok(cfg) => cfg,
            Err(err) => {
                eprintln!("Invalid configuration: {}", err);
                std::process::exit(1);
            }
        };

    println!("Service name: {}", config.app.service_name());
    println!("Service port: {}", config.app.service_port());
}
