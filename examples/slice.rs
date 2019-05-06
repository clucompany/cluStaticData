
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

fn main() {
	println!("#0 {:?}", TEST);
	assert_eq!(TEST.is_true(), false);
	
	let err = TEST.set(&10);
	println!("#1 {:?}, result: {:?}", TEST, err);
	assert_eq!(TEST.is_true(), true);
}
