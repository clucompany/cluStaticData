
use std::fmt::Debug;

#[macro_use]
extern crate cluStaticData;

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

fn main() {
	assert_eq!(TEST.is_true(), false);
	println!("OK #0 {:?}", TEST);
	
	let err = TEST.set(&10);
	assert_eq!(TEST.is_true(), true);
	println!("OK #1 {:?}, result: {:?}", TEST, err);
}
