#![allow(dead_code)]

use client::*;

fn stream(client: &Client, num_g: u64) {
    let size = 1024 * 1024 * 1024 * num_g;
    client.stream(CrashRequest { size }).unwrap();
}

fn main() {
    env_logger::init();

    let client = Client::new("localhost", 50056).unwrap();
    stream(&client, 5)
}