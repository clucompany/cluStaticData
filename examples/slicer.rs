
use cluOnceStatic::StaticTErr;
use cluOnceStatic::StaticData;
use std::fmt::Debug;

#[macro_use]
extern crate cluOnceStatic;



static_data! {
	static ref TEST: &'static (dyn MyTrait + 'static) = &();
}

pub trait MyTrait: Debug + Sync {
	
}

impl MyTrait for () {
	
}

impl MyTrait for usize {
	
}

fn main() -> Result<(), StaticTErr> {
	let bb: &MyTrait = *TEST;
	
	let cc: &StaticData<&'static (dyn MyTrait + 'static)> = &TEST;
	
	TEST.set_once(&10)?;
	
	//println!("{:?}", aa);
	println!("{:?}", bb);
	println!("{:?}", TEST);
	
	TEST.set_once(&20)?;
	
	println!("{:?}", cc);
	
	Ok( () )
}