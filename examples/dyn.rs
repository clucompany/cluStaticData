
#[macro_use]
extern crate cluStaticData;

use std::fmt::Debug;
use cluStaticData::err::StaticErr;


static_data! {
	static ref TEST: &'static (dyn MyTrait + 'static) = &();
}

pub trait MyTrait: Debug + Sync {
	fn data(&self) -> usize;
}

impl MyTrait for () {
	#[inline]
	fn data(&self) -> usize {
		0
	}
}

impl MyTrait for usize {
	#[inline]
	fn data(&self) -> usize {
		*self
	}
}

fn main() -> Result<(), StaticErr<&'static (dyn MyTrait + 'static)>> {
	let _result = TEST.set(&10)?;
	println!("OK {:?}, data: {:?}", TEST, TEST.data());
	
	let err = TEST.set(&20);
	assert_eq!(err.err().unwrap().into_inner().data(), 20);
	println!("OK {:?}, data: {:?}", TEST, TEST.data());
	
	Ok( () )
}