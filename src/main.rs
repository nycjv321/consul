mod client;

use client::ConsulClient;

fn main() {
   let client = ConsulClient{host: String::from("http://127.0.0.1:8500")};
   println!("{:#?}",  client.get(String::from("foo/bar/bar")));
   println!("{}", client.put(String::from("foo/bar/bar"), String::from("foo")))
}