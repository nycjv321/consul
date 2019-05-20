# consul-client
## About
A barebones consul client/lib/sdk for consul. Currently, this lib only supports a few operations on Consul K/V. More are planned and PRs are welcomed!

## Why?
1) Wanted a dead simple API.
2) Wanted a client that supported recent versions of SSL and didn't require me rewriting an existing lib to implement it

## Installing as a Dependency

    [dependencies]
    # as a git dependency
    consul-client = { git = "https://github.com/nycjv321/consul-client" }


## Usage
    extern crate consul_client;
    use consul_client::kv::KVStore;

	fn main() {
   	  let store = KVStore {host: String::from("http://127.0.0.1:8500")};
   	  println!("{:#?}",  store.get(String::from("foo/bar/bar")));
 	  println!("{}", store.put(String::from("foo/bar/bar"), String::from("foo")))
	}