
use crate::err::IgnoreInitErr;
use crate::err::StaticErr;

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

