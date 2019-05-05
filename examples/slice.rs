use cluOnceStatic::StaticTErr;
use std::fmt::Debug;

#[macro_use]
extern crate cluOnceStatic;

static_data! {
	pub(crate) static ref TEST: &'static dyn MyTrait = &();
}

pub trait MyTrait: Debug + Sync {
	fn is_true(&self) -> bool {
		false
	}
}

impl MyTrait for () {
	
}

impl MyTrait for usize {
	#[inline]
	fn is_true(&self) -> bool {
		self > &0
	}
}

fn main() -> Result<(), StaticTErr> {
	println!("{:?}", TEST);
	assert_eq!(TEST.is_true(), false);
	
	TEST.set_once(&10)?;
	println!("{:?}", TEST);
	assert_eq!(TEST.is_true(), true);
	
	Ok( () )
}
