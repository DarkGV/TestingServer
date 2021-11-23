extern crate core_generator;
extern crate server;
use core_generator::launch;
use server::config;


#[launch]
fn test() {
    println!("Hello, world!");
}
