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
*/

#![allow(non_snake_case)]

use crate::err::IgnoreInitErr;
use crate::err::StaticErr;
use crate::set_unsafe::UnsafeInitUnkStaticData;
use crate::set::SetInitUnkStaticData;



use std::sync::atomic::AtomicUsize;
use crate::static_core::AlwaysLockOnce;
use std::fmt::Display;
use std::fmt::Debug;
use std::cell::UnsafeCell;

use std::ops::Deref;


use std::fmt;

pub mod err;
pub (crate) mod static_core;

mod set;
mod set_unsafe;
pub use self::set::*;
pub use self::set_unsafe::*;


#[macro_export]
macro_rules! static_data {
	//static data
	[
		$(#[$($mt:tt)*])*
		static ref $name:ident: $t: ty = $a:expr;	$($tt:tt)*
	] => {
		$(#[$($mt)*])*
		static $name: $crate::StaticData<$t> = $crate::StaticData::new($a);
		
		static_data! {
			$($tt)*
		}
	};
	
	[
		$(#[$($mt:tt)*])*
		pub $(($($at:tt)*))* static ref $name:ident: $t: ty = $a:expr;	$($tt:tt)*
	] => {
		$(#[$($mt)*])*
		pub $(($($at)*))* static $name: $crate::StaticData<$t> = $crate::StaticData::new($a);

		
		static_data! {
			$($tt)*
		}
	};
	
	() => ()
}

pub type StaticData<T>		= UnkStaticData<T, AtomicUsize>;
pub type StaticDataAlwaysLock<T>	= UnkStaticData<T, AlwaysLockOnce>;

pub struct UnkStaticData<T, I> {
	data: UnsafeCell<T>,
	sync_data: I,
}

unsafe impl<T, I> Sync for UnkStaticData<T, I> where T: Sync {}
unsafe impl<T, I> Send for UnkStaticData<T, I> where T: Sync + Send {}

//DONT TRAIT!
impl<T, I> UnkStaticData<T, I> where Self: UnsafeInitUnkStaticData<T> {
	#[inline(always)]
	pub unsafe fn set_box(&self, v: Box<T>) -> Result<(), StaticErr<Box<T>>> {
		UnsafeInitUnkStaticData::set_box(self, v)
	}
	
	#[inline(always)]
	pub unsafe fn set_raw(&self, v: T) -> Result<(), StaticErr<T>> {
		UnsafeInitUnkStaticData::set_raw(self, v)
	}
}




impl<T, I> UnkStaticData<T, I> where Self: SetInitUnkStaticData<T> {
	#[inline(always)]
	pub fn set(&self, v: T) -> Result<(), StaticErr<T>> {
		SetInitUnkStaticData::set(self, v)
	}
	
	#[inline(always)]
	pub fn replace(&self, v: T) -> Result<T, StaticErr<T>> {
		SetInitUnkStaticData::replace(self, v)
	}
	
	#[inline(always)]
	pub unsafe fn unsafe_replace(&self, v: T) -> T {
		SetInitUnkStaticData::unsafe_replace(self, v)
	}
	
	#[inline(always)]
	pub fn get<'a>(&'a self) -> &'a T {
		SetInitUnkStaticData::get(self)
	}
	
	#[inline(always)]
	pub fn ignore_init(&self) -> Result<(), IgnoreInitErr> {
		SetInitUnkStaticData::ignore_init(self)	
	}
	
	#[inline(always)]
	pub fn ignore_init_dont_result(&self) {
		SetInitUnkStaticData::ignore_init_dont_result(self)
	}
	
	#[inline(always)]
	pub fn is_init_state(&self) -> bool {
		SetInitUnkStaticData::is_init_state(self)	
	}
	
	#[inline(always)]
	pub fn is_noinit_state(&self) -> bool {
		SetInitUnkStaticData::is_noinit_state(self)
	}
}


impl<T, I> AsRef<T> for UnkStaticData<T, I> where Self: SetInitUnkStaticData<T> {
	#[inline(always)]
	fn as_ref(&self) -> &T {
		self.get()
	}
}

impl<T, I> Deref for UnkStaticData<T, I> where Self: SetInitUnkStaticData<T> {
	type Target = T;
	
	#[inline(always)]
	fn deref(&self) -> &Self::Target {
		self.get()
	}
}

impl<T, I> Debug for UnkStaticData<T, I> where T: Debug, Self: Deref<Target = T> {
	#[inline(always)]
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		(**self).fmt(f)
	}
}

impl<T, I> Display for UnkStaticData<T, I> where T: Display, Self: Deref<Target = T> {
	#[inline(always)]
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		(**self).fmt(f)
	}
}

