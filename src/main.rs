mod kv;

use kv::KVStore;

fn main() {
   let store = KVStore {host: String::from("http://127.0.0.1:8500")};
   println!("{:#?}",  store.get(String::from("foo/bar/bar")));
   println!("{}", store.put(String::from("foo/bar/bar"), String::from("foo")))
}