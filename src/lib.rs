mod config;
mod database;
mod routes;

use routes::create_routes;
use std::net::Ipv4Addr;

pub struct App {
    port: u16,
    address: Ipv4Addr,
}

impl App {
    pub fn new(port: u16) -> Self {
        let address = Ipv4Addr::new(127, 0, 0, 1);
        Self { port, address }
    }

    pub async fn run(&self) {
        let listener = tokio::net::TcpListener::bind((self.address, self.port)).await;
        match listener {
            Ok(listener) => {
                match axum::serve(listener, create_routes().await).await {
                    Ok(_) => {
                        println!(
                            "\x1b[32m Blog API server running on: http:://{}:{} \x1b[0m",
                            self.address, self.port
                        )
                    }
                    Err(error) => {
                        println!(
                            "\x1b[31m Error starting up the blog API server: {} \x1b[0m",
                            error
                        )
                    }
                };
            }
            Err(error) => {
                println!("\x1b[31m Error creating a TCP listener: {} \x1b[0m", error)
            }
        }
    }
}
