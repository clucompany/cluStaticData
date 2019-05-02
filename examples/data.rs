
#[macro_use]
extern crate cluOnceStatic;


static_data! {
	pub(crate) static ref TEST: TestValue = TestValue::Unk;
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TestValue {
	Unk,
	RuntimeValue(usize),
}

fn main() {
	println!("#1 {:?}", TEST);
	assert_eq!(*TEST, TestValue::Unk);
	
	let result = TEST.set(TestValue::RuntimeValue(10));
	assert_eq!(result, true);
	println!("#2 {:?}", TEST);
	
	let result = TEST.set(TestValue::RuntimeValue(20));
	assert_eq!(result, false);
	assert_eq!(*TEST, TestValue::RuntimeValue(10));
	println!("#3 {:?}", TEST);
	
	let result = TEST.replace(TestValue::Unk);
	assert_eq!(result.is_err(), true);
	println!("#4 {:?}", TEST.replace(TestValue::Unk));
}