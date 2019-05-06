
use crate::err::StaticErr;

pub trait SetInitRawStaticData<T> {
	fn set(&self, v: T) -> Result<(), StaticErr<T>>;
	fn replace(&self, v: T) -> Result<T, StaticErr<T>>;
	
	unsafe fn unsafe_replace(&self, v: T) -> T;
	
	fn get<'a>(&'a self) -> &'a T;
	
	
	
	fn ignore_init(&self) -> Result<(), IgnoreInitErr>;
	fn ignore_init_dont_result(&self);

	
	fn is_init_state(&self) -> bool;
	
	#[inline]
	fn is_noinit_state(&self) -> bool {
		!self.is_init_state()
	}
}


#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum IgnoreInitErr {
	PrevLock,
	AllowLock,
}

impl IgnoreInitErr {
	#[inline]
	pub const fn prev() -> Self {
		Self::PrevLock
	}
	
	#[inline]
	pub const fn allow() -> Self {
		Self::AllowLock	
	}
}
