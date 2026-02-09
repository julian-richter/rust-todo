// Configuration Object with service_name and service_port
#[derive(Debug)]
struct Configuration {
    service_name: String,
    service_port: u16
}


fn main() {
    let config: Configuration = Configuration {
        service_name: String::from("todo"),
        service_port: 8080u16
    };


    // Print manually
    println!("Hello, World!");
    println!("Service: {}", config.service_name);
    println!("Service port: {}", config.service_port);

    // Print using Debug trait
    println!("{:?}", config)
}