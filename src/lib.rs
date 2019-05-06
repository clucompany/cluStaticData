#![allow(non_snake_case)]

#![feature(once_is_completed)]
#![feature(const_fn)]
#![feature(const_raw_ptr_deref)]
#![feature(associated_type_defaults)]
#![feature(specialization)]
#![feature(type_alias_enum_variants)]

use crate::set::IgnoreInitErr;
use crate::err::StaticErr;
use crate::set_unsafe::UnsafeInitRawStaticData;
use crate::set::SetInitRawStaticData;



use std::sync::atomic::AtomicUsize;
use crate::static_core::AlwaysLockOnce;
use std::fmt::Display;
use std::fmt::Debug;
use std::cell::UnsafeCell;

use std::ops::Deref;


use std::fmt;

pub (crate) mod static_core;
pub mod set;
pub mod set_unsafe;
pub mod err;

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

pub type StaticData<T>		= RawStaticData<T, AtomicUsize>;
pub type StaticDataAlwaysLock<T>	= RawStaticData<T, AlwaysLockOnce>;

pub struct RawStaticData<T, I> {
	data: UnsafeCell<T>,
	sync_data: I,
}

unsafe impl<T, I> Sync for RawStaticData<T, I> where T: Sync {}
unsafe impl<T, I> Send for RawStaticData<T, I> where T: Sync + Send {}

//DONT TRAIT!
impl<T, I> RawStaticData<T, I> where Self: UnsafeInitRawStaticData<T> {
	#[inline(always)]
	pub unsafe fn set_box(&self, v: Box<T>) -> Result<(), StaticErr<Box<T>>> {
		UnsafeInitRawStaticData::set_box(self, v)
	}
	
	#[inline(always)]
	pub unsafe fn set_raw(&self, v: T) -> Result<(), StaticErr<T>> {
		UnsafeInitRawStaticData::set_raw(self, v)
	}
}




impl<T, I> RawStaticData<T, I> where Self: SetInitRawStaticData<T> {
	#[inline(always)]
	pub fn set(&self, v: T) -> Result<(), StaticErr<T>> {
		SetInitRawStaticData::set(self, v)
	}
	
	#[inline(always)]
	pub fn replace(&self, v: T) -> Result<T, StaticErr<T>> {
		SetInitRawStaticData::replace(self, v)
	}
	
	#[inline(always)]
	pub unsafe fn unsafe_replace(&self, v: T) -> T {
		SetInitRawStaticData::unsafe_replace(self, v)
	}
	
	#[inline(always)]
	pub fn get<'a>(&'a self) -> &'a T {
		SetInitRawStaticData::get(self)
	}
	
	#[inline(always)]
	pub fn ignore_init(&self) -> Result<(), IgnoreInitErr> {
		SetInitRawStaticData::ignore_init(self)	
	}
	
	#[inline(always)]
	pub fn ignore_init_dont_result(&self) {
		SetInitRawStaticData::ignore_init_dont_result(self)
	}
	
	#[inline(always)]
	pub fn is_init_state(&self) -> bool {
		SetInitRawStaticData::is_init_state(self)	
	}
	
	#[inline(always)]
	pub fn is_noinit_state(&self) -> bool {
		SetInitRawStaticData::is_noinit_state(self)
	}
}


/*
impl<T, O> RawStaticData<&'static T, O> where O: StaticInit, T: 'static {
	pub unsafe fn set_box(&self, v: Box<T>) -> Result<(), StaticTErr> {
		let mut err = Err(StaticTErr::prev());
				
		self.init.raw_lock(|| {
			#[allow(unused_unsafe)]
			unsafe {
				*self.value.get() = &*Box::into_raw(v);
			}
			err = Ok( () );
		});
		err
	}
	
	pub unsafe fn set_raw(&self, v: T) -> Result<(), StaticTErr> {
		let mut err = Err(StaticTErr::prev());
				
		self.init.raw_lock(|| {
			#[allow(unused_unsafe)]
			unsafe {
				let v = Box::new(v);
				*self.value.get() = &*Box::into_raw(v);
			}
			err = Ok( () );
		});
		err
	}
}



impl<T, O> RawStaticData<T, O> where O: StaticInit, T: Copy {
	pub fn set_copy(&self, v: T) -> Result<(), StaticErr<T>> where T: Copy {
		let mut is_err = true;
				
		self.init.raw_lock(|| {
			unsafe {
				*self.value.get() = v;
			}
			is_err = false;
		});
		match is_err {
			false => Ok(()),
			_ => Err( StaticErr::new(v, StaticTErr::prev()) )
		}	
	}
	
	pub fn replace_copy(&self, v: T) -> Result<T, StaticErr<T>> where T: Copy {
		let mut result: Option<T> = None;
				
		self.init.raw_lock(|| {
			result = Some(std::mem::replace(unsafe { &mut *self.value.get() }, v));
		});
		
		match result {
			Some(a) => Ok(a),
			None => Err( StaticErr::new(v, StaticTErr::prev()) )
		}
	}
}

impl<T, O> RawStaticData<T, O> where O: StaticInit {
	pub fn set(&self, v: T) -> Result<(), StaticTErr> {
		let mut err = Err(StaticTErr::prev());
				
		self.init.raw_lock(|| {
			unsafe {
				*self.value.get() = v;
			}
			err = Ok( () );
		});
		err	
	}
	
	
	
	pub fn replace(&self, v: T) -> Option<T> {
		let mut result: Option<T> = None;
				
		self.init.raw_lock(|| {
			result = Some(std::mem::replace(unsafe { &mut *self.value.get() }, v));
		});
		
		result
	}
	
	pub unsafe fn unsafe_replace(&self, v: T) -> T {
		self.ignore_init();
		
		#[allow(unused_unsafe)]
		std::mem::replace(unsafe { &mut *self.value.get() }, v)	
	}
	
	pub const fn raw_get<'a>(&'a self) -> &'a T {
		unsafe{ &*self.value.get() }	
	}
	
	pub fn get<'a>(&'a self) -> &'a T {
		self.ignore_init();
		self.raw_get()
	}
	
	#[inline(always)]
	pub fn is_init_state(&self) -> bool {
		self.init.is_init_state()
	}
	
	#[inline(always)]
	pub fn ignore_init(&self) -> bool {
		self.init.ignore_init()
	}
	
	#[inline(always)]
	pub fn raw_ignore_init(&self) {
		self.init.raw_ignore_init()
	}
	
	
	#[inline]
	pub fn is_noinit_state(&self) -> bool {
		!self.is_init_state()
	}
}
*/


/*impl<T, O> From<T> for RawStaticData<T, O> where O: StaticInit {
	#[inline(always)]
	fn from(a: T) -> Self {
		Self::new(a)
	}
}*/

impl<T, I> AsRef<T> for RawStaticData<T, I> where Self: SetInitRawStaticData<T> {
	#[inline(always)]
	fn as_ref(&self) -> &T {
		self.get()
	}
}

impl<T, I> Deref for RawStaticData<T, I> where Self: SetInitRawStaticData<T> {
	type Target = T;
	
	#[inline(always)]
	fn deref(&self) -> &Self::Target {
		self.get()
	}
}

impl<T, I> Debug for RawStaticData<T, I> where T: Debug, Self: Deref<Target = T> {
	#[inline(always)]
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		(**self).fmt(f)
	}
}

impl<T, I> Display for RawStaticData<T, I> where T: Display, Self: Deref<Target = T> {
	#[inline(always)]
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		(**self).fmt(f)
	}
}




/*
pub const fn set_slice<S, v>(value: &'static S) -> bool where S: StaticValue<V = v> + 'static {
	let mut is_set = false;
	value.INIT.call(|| {
		unsafe {
			LOGGER = log;
		}
		is_set = true;
	});
	is_set
}

pub const fn set<S, v>(log: S) -> bool where S: StaticValue<V = v> + 'static {
	let mut is_set = false;
	LOGGER_INIT.call(move || {
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
	LOGGER_INIT.call(|| {
		unsafe {
			LOGGER = &*Box::into_raw(log);
		}
		is_set = true;
	});
	is_set
}

*/



