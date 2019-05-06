
use crate::err::StaticErr;
use crate::r#unsafe::UnsafeInitRawStaticData;
use crate::StaticTErr;
use std::sync::ONCE_INIT;
use crate::RawStaticData;
use crate::static_core::StaticInit;
use std::sync::Once;
use std::cell::UnsafeCell;

impl StaticInit for Once {
	#[inline(always)]
	fn raw_lock<F: FnOnce()>(&self, f: F) {
		self.call_once(f)
	}
	
	
	#[inline(always)]
	fn is_init_state(&self) -> bool {
		self.is_completed()
	}
}

impl<T> RawStaticData<T, Once> {
	#[inline]
	pub const fn new(a: T) -> Self {
		Self {
			value: UnsafeCell::new(a),
			init: ONCE_INIT,
		}
	}
}


impl<T> UnsafeInitRawStaticData<T> for RawStaticData<&'static T, Once> {
	type Box_Ok = ();
	type Box_Err = StaticErr<Box<T>>;
	
	type Raw_Ok = ();
	type Raw_Err = StaticErr<T>;
	
	unsafe fn set_box(&self, v: Box<T>) -> Result<Self::Box_Ok, Self::Box_Err> {
		let mut err = Err(Self::Err::prev());
		
		self.init.raw_lock(|| {
			#[allow(unused_unsafe)]
			unsafe {
				*self.value.get() = &*Box::into_raw(v);
			}
			err = Ok( () );
		});
		err
	}
	
	unsafe fn set_raw(&self, v: T) -> Result<Self::Raw_Ok, Self::Raw_Err> {
		let mut err = Err(Self::Err::prev());
		
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
