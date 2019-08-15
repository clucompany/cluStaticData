//Copyright 2019 #UlinProject Денис Котляров

//Licensed under the Apache License, Version 2.0 (the "License");
//you may not use this file except in compliance with the License.
//You may obtain a copy of the License at

//	   http://www.apache.org/licenses/LICENSE-2.0

//Unless required by applicable law or agreed to in writing, software
//distributed under the License is distributed on an "AS IS" BASIS,
//WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//See the License for the specific language governing permissions and
// limitations under the License.

//#Ulin Project 1819
//

/*!

Initializers of static values. Manual initialization (using safe functions and zero cost when accessing an object) or automatic initialization during accessing an object (there is no zero cost when accessing an object; you must set the correct default value).

1. Manual initialization of static data.
2. Automatic initialization of static data.


# Use (Manual initialization of static data)

```
#[macro_use]
extern crate cluStaticData;

use std::fmt::Debug;
use cluStaticData::err::StaticErr;


static_data! {
	static ref TEST: &'static (dyn MyTrait + 'static) = &();
}

pub trait MyTrait: Debug + Sync {
	fn data(&self) -> usize;
}

impl MyTrait for () {
	#[inline]
	fn data(&self) -> usize {
		0
	}
}

impl MyTrait for usize {
	#[inline]
	fn data(&self) -> usize {
		*self
	}
}

fn main() -> Result<(), StaticErr<&'static (dyn MyTrait + 'static)>> {
	let _result = TEST.set(&10)?;
	println!("OK {:?}, data: {:?}", TEST, TEST.data());
	
	let err = TEST.set(&20);
	assert_eq!(err.err().unwrap().into_inner().data(), 20);
	println!("OK {:?}, data: {:?}", TEST, TEST.data());
	
	Ok( () )
}
```

# Use 2 (Manual initialization of static data)

```
#[macro_use]
extern crate cluStaticData;
use cluStaticData::err::StaticErr;

static_data! {
	pub(crate) static ref TEST: TestValue = TestValue::Unk;
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TestValue {
	Unk,
	RuntimeValue(usize),
}

fn main() {
	assert_eq!(*TEST, TestValue::Unk);
	println!("OK #1 {:?}", TEST);
	
	let result = TEST.set(TestValue::RuntimeValue(10));
	assert_eq!(result.is_ok(), true);
	println!("OK #2 {:?}", TEST);
	
	let result = TEST.set(TestValue::RuntimeValue(20));
	assert_eq!(result.is_ok(), false);
	assert_eq!(*TEST, TestValue::RuntimeValue(10));
	println!("OK #3 {:?}", TEST);
	
	let result = TEST.replace(TestValue::Unk);
	assert_eq!(result, Err(StaticErr::prev(TestValue::Unk)));
	println!("OK #4 {:?}", result);
}
```

# Use 3 (Automatic initialization of static data)

```
#[macro_use]
extern crate cluStaticData;

use std::collections::HashMap;

static_data! {
	pub(crate) static ref +runtime HASH_MAP: HashMap<String, String> = {
		let mut hash_map = HashMap::new();
		hash_map.insert("test".to_string(), "b".to_string());
		hash_map.insert("test2".to_string(), "b2".to_string());

		hash_map
	};
	static ref +runtime HASH_MAP2: usize = 0;
}

fn main() {
	println!("{:?}", HASH_MAP);
}
```

# License

Copyright 2019 #UlinProject Denis Kotlyarov (Денис Котляров)

Licensed under the Apache License, Version 2.0
*/

#![allow(non_snake_case)]

pub mod once_runtime;
pub mod once_const_static;


#[macro_use]
mod macros;
pub use self::macros::*;

pub mod err;
