
use super::*;
use bench::Bencher;

fn stream(client: &Client, num_g: u64) {
    let size = 1024 * 1024 * 1024 * num_g;
    client.stream(CrashRequest { size }).unwrap();
}

#[bench]
fn stream_5g(b: &mut Bencher) {
    let client = Client::new("localhost", 50056).unwrap();
    b.iter(|| stream(&client, 5));
}

#[bench]
fn stream_10g(b: &mut Bencher) {
    let client = Client::new("localhost", 50056).unwrap();
    b.iter(|| stream(&client, 10));
}
