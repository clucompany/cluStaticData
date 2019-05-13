
pub mod core {
	mod always_lock;
	pub use self::always_lock::*;
	
	mod atomic_lock;
	pub use self::atomic_lock::*;
}

pub mod generic;

#[macro_use]
mod macros;
pub use self::macros::*;

use ::core::fmt::Debug;
use ::core::fmt::Display;
use ::core::ops::Deref;
use ::core::sync::atomic::AtomicU8;
use ::core::cell::UnsafeCell;
use ::core::fmt;

use crate::once_const_static::generic::GenericStaticData;
use crate::once_const_static::generic::UnsafeGenericStaticData;
use crate::once_const_static::core::AlwaysLockOnce;

use crate::err::StaticErr;
use crate::err::IgnoreInitErr;




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



