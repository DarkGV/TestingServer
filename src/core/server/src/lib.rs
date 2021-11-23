pub mod config;
use config::Configuration;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::thread;

pub fn launch(serverconfiguration: Configuration, handlefunction: &'static (dyn Fn() + Sync)) {
    let port = serverconfiguration.ServerConfig.Port;
    let serverlisten_address = get_address(serverconfiguration.ServerConfig.Host);
    let mut fulladdress = String::new();
    fulladdress.push_str(&serverlisten_address);
    fulladdress.push_str(":");
    fulladdress.push_str(&(port.to_string()));

    let listensocket = TcpListener::bind(fulladdress.to_string()).unwrap();
    for incoming in listensocket.incoming() {
        match incoming {
            Ok(client_stream) => {
                thread::spawn(move || {
                    handlefunction();
                    client_stream.shutdown(Shutdown::Both);
                });
                println!("Connection Accepted");
            },
            Err(_) =>
                println!("Connection Failed"),
                
        }
    }
}


// Internal functions
fn get_address(host: Option<String>) -> String {
    let (local, any) = (String::from("local"), String::from("any"));
    let address = 
        match host {
            Some(local) => "127.0.0.1",
            Some(any) => "0.0.0.0",
            _ => "127.0.0.1",
        };
    String::from(address)
}

fn handle_client(clientstream: TcpStream, handlefunction: &'static dyn Fn()) {
    handlefunction();
    clientstream.shutdown(Shutdown::Both);
}
