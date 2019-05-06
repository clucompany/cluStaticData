
#[macro_use]
extern crate cluOnceStatic;

use cluOnceStatic::StaticData;
use std::fmt::Debug;
use cluOnceStatic::err::StaticErr;


static_data! {
	static ref TEST: &'static (dyn MyTrait + 'static) = &();
}

pub trait MyTrait: Debug + Sync {
	
}

impl MyTrait for () {
	
}

impl MyTrait for usize {
	
}

fn main() -> Result<(), StaticErr<&'static (dyn MyTrait + 'static)>> {
	let bb: &MyTrait = *TEST;
	
	let _cc: &StaticData<&'static (dyn MyTrait + 'static)> = &TEST;
	
	let result = TEST.set(&10)?;
	
	//println!("{:?}", aa);
	println!("{:?}", bb);
	println!("{:?}", TEST);
	println!("{:?}", result);
	
	let result = TEST.set(&20);
	println!("{:?}", result);
	
	Ok( () )
}