
#[macro_use]
extern crate cluStaticData;

use std::collections::HashMap;

//cargo run --example runtime --features "once_runtime, trivial_bounds"

static_data! {
	pub(crate) static ref +runtime HASH_MAP: HashMap<String, String> = {
		let mut hash_map = HashMap::new();
		hash_map.insert("test".to_string(), "b".to_string());
		hash_map.insert("test2".to_string(), "b2".to_string());

		hash_map
	};
	static ref +runtime VEC: Vec<usize> = {
		let mut vec = Vec::with_capacity(12);
		for a in 0..10 {
			vec.push(a);
		}
		vec	
	};
	static ref +runtime STR: String = String::from("check");
	static ref +runtime HASH_MAP2: usize = 0;
}

fn main() {
	println!("{:?}", HASH_MAP);
	println!("{:?}", VEC);
	
	#[cfg(feature = "nightly")]
	println!("{}", STR);
}