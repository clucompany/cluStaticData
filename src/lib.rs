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

# License

Copyright 2019 #UlinProject Denis Kotlyarov (Денис Котляров)

Licensed under the Apache License, Version 2.0
*/

#![allow(non_snake_case)]

use std::sync::atomic::AtomicU8;
use crate::err::IgnoreInitErr;
use crate::err::StaticErr;
use crate::static_core::AlwaysLockOnce;
use std::fmt::Display;
use std::fmt::Debug;
use std::cell::UnsafeCell;
use std::ops::Deref;
use std::fmt;

pub mod err;

#[macro_use]
mod macros;
pub use self::macros::*;

pub mod static_core {
	mod always_lock;
	pub use self::always_lock::*;
	
	mod atomic_lock;
	pub use self::atomic_lock::*;
}


pub type StaticData<T>			= UnkStaticData<T, AtomicU8>;
pub type StaticDataAlwaysLock<T>	= UnkStaticData<T, AlwaysLockOnce>;

pub struct UnkStaticData<T, I> {
	data: UnsafeCell<T>,
	sync_data: I,
}

unsafe impl<T, I> Sync for UnkStaticData<T, I> where T: Sync {}
unsafe impl<T, I> Send for UnkStaticData<T, I> where T: Sync + Send {}

//DONT TRAIT!
impl<T, I> UnkStaticData<T, I> where Self: UnsafeGenericStaticData<T> {
	#[inline(always)]
	pub unsafe fn set_box(&self, v: Box<T>) -> Result<(), StaticErr<Box<T>>> {
		UnsafeGenericStaticData::set_box(self, v)
	}
	
	#[inline(always)]
	pub unsafe fn set_raw(&self, v: T) -> Result<(), StaticErr<T>> {
		UnsafeGenericStaticData::set_raw(self, v)
	}
}

impl<T, I> UnkStaticData<T, I> where Self: GenericStaticData<T> {
	#[inline(always)]
	pub fn set(&self, v: T) -> Result<(), StaticErr<T>> {
		GenericStaticData::set(self, v)
	}
	
	#[inline(always)]
	pub fn replace(&self, v: T) -> Result<T, StaticErr<T>> {
		GenericStaticData::replace(self, v)
	}
	
	#[inline(always)]
	pub unsafe fn unsafe_replace(&self, v: T) -> T {
		GenericStaticData::unsafe_replace(self, v)
	}
	
	#[inline(always)]
	pub fn get<'a>(&'a self) -> &'a T {
		GenericStaticData::get(self)
	}
	
	#[inline(always)]
	pub fn ignore_initialize(&self) -> Result<(), IgnoreInitErr> {
		GenericStaticData::ignore_initialize(self)	
	}
	
	#[inline(always)]
	pub fn ignore_initialize_dont_result(&self) {
		GenericStaticData::ignore_initialize_dont_result(self)
	}
	
	#[inline(always)]
	pub fn is_init_state(&self) -> bool {
		GenericStaticData::is_init_state(self)	
	}
	
	#[inline(always)]
	pub fn is_noinit_state(&self) -> bool {
		GenericStaticData::is_noinit_state(self)
	}
}


impl<T, I> AsRef<T> for UnkStaticData<T, I> where Self: GenericStaticData<T> {
	#[inline(always)]
	fn as_ref(&self) -> &T {
		self.get()
	}
}

impl<T, I> Deref for UnkStaticData<T, I> where Self: GenericStaticData<T> {
	type Target = T;
	
	#[inline(always)]
	fn deref(&self) -> &Self::Target {
		self.get()
	}
}

impl<T, I> Debug for UnkStaticData<T, I> where T: Debug, Self: GenericStaticData<T> {
	#[inline(always)]
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		self.get().fmt(f)
	}
}

impl<T, I> Display for UnkStaticData<T, I> where T: Display, Self: GenericStaticData<T> {
	#[inline(always)]
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		self.get().fmt(f)
	}
}



pub trait GenericStaticData<T> {
	fn set(&self, v: T) -> Result<(), StaticErr<T>>;
	fn replace(&self, v: T) -> Result<T, StaticErr<T>>;
	
	unsafe fn unsafe_replace(&self, v: T) -> T;
	
	fn get<'a>(&'a self) -> &'a T;
	
	
	fn ignore_initialize(&self) -> Result<(), IgnoreInitErr>;
	fn ignore_initialize_dont_result(&self);

	
	fn is_init_state(&self) -> bool;
	
	#[inline]
	fn is_noinit_state(&self) -> bool {
		!self.is_init_state()
	}
}

pub trait UnsafeGenericStaticData<T> {
	unsafe fn set_box(&self, v: Box<T>) -> Result<(), StaticErr<Box<T>>>;
	unsafe fn set_raw(&self, v: T) -> Result<(), StaticErr<T>>;
}

