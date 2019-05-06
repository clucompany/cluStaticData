
use std::ops::Deref;
use std::ops::DerefMut;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum StaticTErr {
	PrevLock,
	AllowLock,
	UnkState,
}

impl StaticTErr {
	#[inline]
	pub const fn prev() -> Self {
		Self::PrevLock
	}
	
	#[inline]
	pub const fn allow() -> Self {
		Self::AllowLock
	}
	
	#[inline]
	pub const fn unk() -> Self {
		Self::UnkState
	}
}



#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct StaticErr<T> {
	data:		T,
	r#type:	StaticTErr,
}

impl<T> From<T> for StaticErr<T> {
	#[inline(always)]
	fn from(a: T) -> Self {
		Self::unk(a)
	}
}

impl<T> StaticErr<T> {
	#[inline]
	pub const fn new(arg: T, err: StaticTErr) -> Self {
		Self {
			data:		arg,
			r#type:	err,
		}
	}
	
	#[inline]
	pub const fn prev(arg: T) -> Self {
		Self::new(arg, StaticTErr::prev())
	}
	
	#[inline]
	pub const fn allow(arg: T) -> Self {
		Self::new(arg, StaticTErr::allow())
	}
	
	#[inline]
	pub const fn unk(arg: T) -> Self {
		Self::new(arg, StaticTErr::unk())
	}
	
	#[inline]
	pub fn into_inner(self) -> T {
		self.data
	}
	
	#[inline]
	pub fn into_type(self) -> StaticTErr {
		self.r#type
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
