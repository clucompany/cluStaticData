
#[macro_use]
extern crate cluStaticData;

use cluStaticData::err::StaticErr;

static_data! {
	pub(crate) static ref DROPPER: MyDrop = MyDrop(0);
}

#[derive(Debug, PartialEq)]
pub struct MyDrop(usize);

impl Drop for MyDrop {
	fn drop(&mut self) {
		println!("line: {}, drop: MyDrop({})", line!(), self.0);	
	}
}

fn main() -> Result<(), StaticErr<MyDrop>> {
	DROPPER.set(MyDrop(1))?;
	println!("OK #0 this_value {:?}", DROPPER);
	
	let err = DROPPER.set(MyDrop(2));
	assert_eq!(err, Err(StaticErr::prev(MyDrop(2))) );
	println!("OK #1 this_value {:?}", DROPPER);
	
	let err = DROPPER.set(MyDrop(3));
	assert_eq!(err, Err(StaticErr::prev(MyDrop(3))) );
	println!("OK #2 this_value {:?}", DROPPER);
	
	Ok( () )
}