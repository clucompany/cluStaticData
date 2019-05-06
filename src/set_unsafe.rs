
use crate::err::StaticErr;

pub trait UnsafeInitUnkStaticData<T> {
	unsafe fn set_box(&self, v: Box<T>) -> Result<(), StaticErr<Box<T>>>;
	unsafe fn set_raw(&self, v: T) -> Result<(), StaticErr<T>>;
}
