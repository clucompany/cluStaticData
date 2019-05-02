
#[macro_use]
extern crate cluOnceStatic;


static_data! {
	pub(crate) static ref DROPPER: MyDrop = MyDrop(0);
}

#[derive(Debug)]
pub struct MyDrop(usize);

impl Drop for MyDrop {
	fn drop(&mut self) {
		println!("drop {}", self.0);	
	}
}

fn main() {
	DROPPER.set(MyDrop(1));
	println!("#0 {:?}", DROPPER);
	
	DROPPER.set(MyDrop(2));
	println!("#1 {:?}", DROPPER);
	
	DROPPER.set(MyDrop(3));
	println!("#2 {:?}", DROPPER);
}