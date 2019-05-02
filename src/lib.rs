#![allow(non_snake_case)]

#![feature(maybe_uninit)]
#![feature(once_is_completed)]
#![feature(const_fn)]
#![feature(const_raw_ptr_deref)]

use std::fmt::Display;
use std::fmt::Debug;
use std::cell::UnsafeCell;
use std::ops::DerefMut;
use std::ops::Deref;
use std::sync::Once;
use std::sync::ONCE_INIT;
use std::fmt;


#[macro_export]
macro_rules! static_data {
	
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


pub struct StaticData<T> {
	value: UnsafeCell<T>
}

impl<T> StaticData<T> {
	#[inline]
	pub const fn new(a: T) -> Self {
		Self {
			value: UnsafeCell::new(a)
		}
	}
	
	#[inline(always)]
	fn as_init(&self) -> &Once {
		static INIT: Once = ONCE_INIT;
		&INIT
	}
	
	pub fn set_slice(&self, v: T) -> Result<(), StaticErr<T>> where T: Copy {
		match self.set(v) {
			true => Ok(()),
			_ => Err( StaticErr::new(v, StaticTErr::PrevLock) )
		}
	}
	
	pub fn set(&self, v: T) -> bool {
		let mut is_set = false;
				
		self.as_init().call_once(|| {
			unsafe{
				*self.value.get() = v;
			}
			
			is_set = true;
		});
		is_set
	}
	
	pub fn replace(&self, v: T) -> Result<T, StaticErr<T>> where T: Copy {
		let mut result: Option<T> = None;
				
		self.as_init().call_once(|| {
			result = Some(std::mem::replace(unsafe { &mut *self.value.get() }, v));
		});
		
		match result {
			Some(a) => Ok(a),
			None => Err( StaticErr::new(v, StaticTErr::PrevLock) )
		}
	}
	
	
	#[inline(always)]
	pub const fn get<'a>(&'a self) -> &'a T {
		unsafe{ &*self.value.get() }
	}
	
	#[inline(always)]
	pub fn is_set(&self) -> bool {
		self.as_init().is_completed()
	}
}

unsafe impl<T> Sync for StaticData<T> {}

impl<T> From<T> for StaticData<T> {
	#[inline(always)]
	fn from(a: T) -> Self {
		Self::new(a)
	}
}

impl<T> AsRef<T> for StaticData<T> {
	#[inline(always)]
	fn as_ref(&self) -> &T {
		self.get()
	}
}

impl<T> Deref for StaticData<T> {
	type Target = T;
	
	#[inline(always)]
	fn deref(&self) -> &Self::Target {
		self.get()
	}
}

impl<T> Debug for StaticData<T> where T: Debug {
	#[inline(always)]
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		(**self).fmt(f)
	}
}

impl<T> Display for StaticData<T> where T: Display {
	#[inline(always)]
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		(**self).fmt(f)
	}
}


#[derive(Debug, Clone, PartialEq)]
pub enum StaticTErr {
	PrevLock,
}

impl Default for StaticTErr {
	#[inline]
	fn default() -> Self {
		StaticTErr::PrevLock
	}
}

#[derive(Debug)]
pub struct StaticErr<T> {
	r#arg:	T,
	r#type:	StaticTErr,
}


impl<T> StaticErr<T> {
	#[inline]
	pub const fn new(arg: T, err: StaticTErr) -> Self {
		Self {
			r#arg:	arg,
			r#type:	err,
		}
	}
	
	#[inline]
	pub fn into_inner(self) -> T {
		self.r#arg
	}
	
	#[inline(always)]
	pub const fn as_type(&self) -> &StaticTErr {
		&self.r#type
	}
	
	#[inline(always)]
	pub const fn as_inner(&self) -> &T {
		&self.r#arg	
	}
}


impl<T> From<(T, StaticTErr)> for StaticErr<T> {
	#[inline(always)]
	fn from((v, t): (T, StaticTErr)) -> Self {
		Self::new(v, t)
	}
}

impl<T> Deref for StaticErr<T> {
	type Target = StaticTErr;
	
	#[inline(always)]
	fn deref(&self) -> &Self::Target {
		&self.r#type
	}
}

impl<T> DerefMut for StaticErr<T> {	
	#[inline(always)]
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.r#type
	}
}


/*
pub const fn set_slice<S, v>(value: &'static S) -> bool where S: StaticValue<V = v> + 'static {
	let mut is_set = false;
	value.INIT.call_once(|| {
		unsafe {
			LOGGER = log;
		}
		is_set = true;
	});
	is_set
}

pub const fn set<S, v>(log: S) -> bool where S: StaticValue<V = v> + 'static {
	let mut is_set = false;
	LOGGER_INIT.call_once(move || {
		unsafe {
			let log = Box::new(log);
			LOGGER = &*Box::into_raw(log);
		}
		is_set = true;
	});
	is_set
}


pub const fn set_boxed<S, v>(log: Box<S>) -> bool where S: StaticValue<V = v> + 'static {
	let mut is_set = false;
	LOGGER_INIT.call_once(|| {
		unsafe {
			LOGGER = &*Box::into_raw(log);
		}
		is_set = true;
	});
	is_set
}

*/