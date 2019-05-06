
#[macro_use]
extern crate cluStaticData;

use cluStaticData::err::StaticErr;

static_data! {
	pub(crate) static ref DROPPER: MyDrop = MyDrop(0);
}

#[derive(Debug)]
pub struct MyDrop(usize);

impl Drop for MyDrop {
	fn drop(&mut self) {
		println!("drop MyDrop({})", self.0);	
	}
}

fn main() -> Result<(), StaticErr<MyDrop>> {
	DROPPER.set(MyDrop(1))?;
	println!("this_value {:?} #0", DROPPER);
	
	DROPPER.set(MyDrop(2))?;
	println!("this_value {:?} #1", DROPPER);
	
	DROPPER.set(MyDrop(3))?;
	println!("this_value {:?} #2", DROPPER);
	
	Ok( () )
}