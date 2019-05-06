# cluStaticData
[![Build Status](https://travis-ci.org/clucompany/cluStaticData.svg?branch=master)](https://travis-ci.org/clucompany/cluStaticData)
[![Apache licensed](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](./LICENSE)
[![crates.io](http://meritbadge.herokuapp.com/cluStaticData)](https://crates.io/crates/cluStaticData)
[![Documentation](https://docs.rs/cluStaticData/badge.svg)](https://docs.rs/cluStaticData)

Methods for describing and creating static values. Implemented static value constructors executed in real-time.

# Use (static trait data)

```

use std::fmt::Debug;

#[macro_use]
extern crate cluStaticData;

static_data! {
	pub(crate) static ref TEST: &'static dyn MyTrait = &();
}

pub trait MyTrait: Debug + Sync {
	fn is_true(&self) -> bool {
		false
	}
}

impl MyTrait for () {
	
}

impl MyTrait for usize {
	#[inline]
	fn is_true(&self) -> bool {
		self > &0
	}
}

fn main() {
	println!("#0 {:?}", TEST);
	assert_eq!(TEST.is_true(), false);
	
	let err = TEST.set(&10);
	println!("#1 {:?}, result: {:?}", TEST, err);
	assert_eq!(TEST.is_true(), true);
}
```

# Use (static data, unk type)

```

#[macro_use]
extern crate cluStaticData;
use cluStaticData::err::StaticErr;

static_data! {
	pub (crate) static ref TEST: TestValue = TestValue::Unk;
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TestValue {
	Unk,
	RuntimeValue(usize),
}

fn main() {
	println!("#1 {:?}", TEST);
	assert_eq!(*TEST, TestValue::Unk);
	
	let result = TEST.set(TestValue::RuntimeValue(10));
	assert_eq!(result.is_ok(), true);
	println!("#2 {:?}", TEST);
	
	let result = TEST.set(TestValue::RuntimeValue(20));
	assert_eq!(result.is_ok(), false);
	assert_eq!(*TEST, TestValue::RuntimeValue(10));
	println!("#3 {:?}", TEST);
	
	let result = TEST.replace(TestValue::Unk);
	assert_eq!(result, Err(StaticErr::prev(TestValue::Unk)));
	println!("#4 {:?}", result);
}
```

# License

Copyright 2019 #UlinProject Denis Kotlyarov (Денис Котляров)

Licensed under the Apache License, Version 2.0
