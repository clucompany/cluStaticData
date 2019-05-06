
use std::ops::Deref;
use std::ops::DerefMut;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum StaticInitErr {
	PrevLock,
	AllowLock,
	UnkState,
}

impl StaticInitErr {
	#[inline]
	pub const fn prev() -> Self {
		StaticInitErr::PrevLock
	}
	
	#[inline]
	pub const fn allow() -> Self {
		StaticInitErr::AllowLock
	}
	
	#[inline]
	pub const fn unk() -> Self {
		StaticInitErr::UnkState
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
		IgnoreInitErr::PrevLock
	}
	
	#[inline]
	pub const fn allow() -> Self {
		IgnoreInitErr::AllowLock	
	}
}



#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct StaticErr<T> {
	data:		T,
	r#type:	StaticInitErr,
}

impl<T> From<T> for StaticErr<T> {
	#[inline(always)]
	fn from(a: T) -> Self {
		Self::unk(a)
	}
}

impl<T> StaticErr<T> {
	#[inline]
	pub const fn new(arg: T, err: StaticInitErr) -> Self {
		Self {
			data:		arg,
			r#type:	err,
		}
	}
	
	#[inline]
	pub const fn prev(arg: T) -> Self {
		Self::new(arg, StaticInitErr::prev())
	}
	
	#[inline]
	pub const fn allow(arg: T) -> Self {
		Self::new(arg, StaticInitErr::allow())
	}
	
	#[inline]
	pub const fn unk(arg: T) -> Self {
		Self::new(arg, StaticInitErr::unk())
	}
	
	#[inline]
	pub fn into_inner(self) -> T {
		self.data
	}
	
	#[inline]
	pub fn into_type(self) -> StaticInitErr {
		self.r#type
	}
	
	#[inline(always)]
	pub const fn as_type(&self) -> &StaticInitErr {
		&self.r#type
	}
	
	#[inline(always)]
	pub const fn as_inner(&self) -> &T {
		&self.data
	}
}


impl<T> From<(T, StaticInitErr)> for StaticErr<T> {
	#[inline(always)]
	fn from((v, t): (T, StaticInitErr)) -> Self {
		Self::new(v, t)
	}
}

impl<T> Deref for StaticErr<T> {
	type Target = StaticInitErr;
	
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
