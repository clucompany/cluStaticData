# cluStaticData
[![Build Status](https://travis-ci.org/clucompany/cluStaticData.svg?branch=master)](https://travis-ci.org/clucompany/cluStaticData)
[![Apache licensed](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](./LICENSE)
[![crates.io](http://meritbadge.herokuapp.com/cluStaticData)](https://crates.io/crates/cluStaticData)
[![Documentation](https://docs.rs/cluStaticData/badge.svg)](https://docs.rs/cluStaticData)

Methods for describing and creating static values. Implemented static value constructors executed in real-time.

# Use (static trait data)

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

# Use (static data, unk type)

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

# Use Runtime

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
