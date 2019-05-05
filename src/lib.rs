#![allow(non_snake_case)]

#![feature(once_is_completed)]
#![feature(const_fn)]
#![feature(const_raw_ptr_deref)]

use crate::static_core::AlwaysLockOnce;
use std::fmt::Display;
use std::fmt::Debug;
use std::cell::UnsafeCell;
use std::ops::DerefMut;
use std::ops::Deref;
use std::sync::Once;
use std::sync::ONCE_INIT;
use std::fmt;

pub (crate) mod static_core;

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

pub type StaticData<T>			= RawStaticData<T, Once>;
pub type StaticDataAlwaysLock<T>	= RawStaticData<T, AlwaysLockOnce>;

pub struct RawStaticData<T, Once> where Once: StaticOnce {
	value: UnsafeCell<T>,
	once: Once,
}

unsafe impl<T, O> Sync for RawStaticData<T, O> where T: Sync, O: StaticOnce {}
unsafe impl<T, O> Send for RawStaticData<T, O> where T: Sync + Send, O: StaticOnce {}

impl<T> RawStaticData<T, Once> {
	#[inline]
	pub const fn new(a: T) -> Self {
		//static INIT: Once = ONCE_INIT;
		Self {
			value: UnsafeCell::new(a),
			once: ONCE_INIT,
		}
	}
}


impl<'a, T, O> RawStaticData<&'a T, O> where O: StaticOnce, T: 'static {
	pub unsafe fn box_set_once(&self, v: Box<T>) -> Result<(), StaticTErr> {
		let mut err = Err(StaticTErr::PrevLock);
				
		self.once.raw_lock_once(|| {
			#[allow(unused_unsafe)]
			unsafe {
				*self.value.get() = &*Box::into_raw(v);
			}
			err = Ok( () );
		});
		err
	}
	pub unsafe fn set_raw_once(&self, v: T) -> Result<(), StaticTErr> {
		let mut err = Err(StaticTErr::PrevLock);
				
		self.once.raw_lock_once(|| {
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


impl<T, O> RawStaticData<T, O> where O: StaticOnce {
	pub fn set_once(&self, v: T) -> Result<(), StaticTErr> {
		let mut err = Err(StaticTErr::PrevLock);
				
		self.once.raw_lock_once(|| {
			unsafe {
				*self.value.get() = v;
			}
			err = Ok( () );
		});
		err	
	}
	
	pub fn set_once_copy(&self, v: T) -> Result<(), StaticErr<T>> where T: Copy {
		let mut is_err = true;
				
		self.once.raw_lock_once(|| {
			unsafe {
				*self.value.get() = v;
			}
			is_err = false;
		});
		match is_err {
			false => Ok(()),
			_ => Err( StaticErr::new(v, StaticTErr::PrevLock) )
		}	
	}
	
	pub fn replace_once(&self, v: T) -> Option<T> {
		let mut result: Option<T> = None;
				
		self.once.raw_lock_once(|| {
			result = Some(std::mem::replace(unsafe { &mut *self.value.get() }, v));
		});
		
		result
	}
	
	pub fn replace_once_copy(&self, v: T) -> Result<T, StaticErr<T>> where T: Copy {
		let mut result: Option<T> = None;
				
		self.once.raw_lock_once(|| {
			result = Some(std::mem::replace(unsafe { &mut *self.value.get() }, v));
		});
		
		match result {
			Some(a) => Ok(a),
			None => Err( StaticErr::new(v, StaticTErr::PrevLock) )
		}
	}
	
	
	
	
	pub unsafe fn replace(&self, v: T) -> T {
		self.ignore_init_once();
		
		#[allow(unused_unsafe)]
		std::mem::replace(unsafe { &mut *self.value.get() }, v)	
	}
	
	
	pub const fn get<'a>(&'a self) -> &'a T {
		unsafe{ &*self.value.get() }	
	}
	
	pub fn get_once<'a>(&'a self) -> &'a T {
		self.ignore_init_once();
		self.get()
	}
	
	#[inline(always)]
	pub fn is_init_state(&self) -> bool {
		self.once.is_init_state()
	}
	
	#[inline(always)]
	pub fn ignore_init_once(&self) -> bool {
		self.once.ignore_init_once()
	}
	
	#[inline(always)]
	pub fn raw_ignore_init_once(&self) {
		self.once.raw_ignore_init_once()
	}
	
	
	#[inline]
	pub fn is_noinit_state(&self) -> bool {
		!self.is_init_state()
	}
}



/*impl<T, O> From<T> for RawStaticData<T, O> where O: StaticOnce {
	#[inline(always)]
	fn from(a: T) -> Self {
		Self::new(a)
	}
}*/

impl<T, O> AsRef<T> for RawStaticData<T, O> where O: StaticOnce {
	#[inline(always)]
	fn as_ref(&self) -> &T {
		self.get()
	}
}

impl<T, O> Deref for RawStaticData<T, O> where O: StaticOnce {
	type Target = T;
	
	#[inline(always)]
	fn deref(&self) -> &Self::Target {
		self.get()
	}
}

impl<T, O> Debug for RawStaticData<T, O> where T: Debug, O: StaticOnce {
	#[inline(always)]
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		(**self).fmt(f)
	}
}

impl<T, O> Display for RawStaticData<T, O> where T: Display, O: StaticOnce {
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
	data:	T,
	r#type:	StaticTErr,
}


impl<T> StaticErr<T> {
	#[inline]
	pub const fn new(arg: T, err: StaticTErr) -> Self {
		Self {
			data:	arg,
			r#type:	err,
		}
	}
	
	#[inline]
	pub fn into_inner(self) -> T {
		self.data
	}
	
	#[inline(always)]
	pub const fn as_type(&self) -> &StaticTErr {
		&self.r#type
	}
	
	#[inline(always)]
	pub const fn as_inner(&self) -> &T {
		&self.data
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




pub trait StaticOnce {
	fn raw_lock_once<F: FnOnce()>(&self, f: F);
	

	#[inline]
	fn ignore_init_once(&self) -> bool {
		let mut is_init = false;
		self.raw_lock_once(|| {
			is_init = true;
		});
		
		is_init
	}
	
	#[inline]
	fn raw_ignore_init_once(&self) {
		self.raw_lock_once(|| {});
	}
	
	fn is_init_state(&self) -> bool;
}


impl<'a, 'c, A> StaticOnce for &'a A where A: StaticOnce + 'c {
	#[inline(always)]
	fn raw_lock_once<F: FnOnce()>(&self, f: F) {
		A::raw_lock_once(self, f)
	}

	#[inline(always)]
	fn ignore_init_once(&self) -> bool {
		A::ignore_init_once(self)
	}
	
	#[inline(always)]
	fn raw_ignore_init_once(&self) {
		A::raw_ignore_init_once(self)
	}
	
	#[inline(always)]
	fn is_init_state(&self) -> bool {
		A::is_init_state(self)
	}
}

impl<'a, A> StaticOnce for Box<A> where A: StaticOnce + 'a {
	#[inline(always)]
	fn raw_lock_once<F: FnOnce()>(&self, f: F) {
		A::raw_lock_once(self, f)
	}

	#[inline(always)]
	fn ignore_init_once(&self) -> bool {
		A::ignore_init_once(self)
	}
	
	#[inline(always)]
	fn raw_ignore_init_once(&self) {
		A::raw_ignore_init_once(self)
	}
	
	#[inline(always)]
	fn is_init_state(&self) -> bool {
		A::is_init_state(self)
	}
}
