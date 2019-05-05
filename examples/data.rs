
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
	
	let result = TEST.set_once(TestValue::RuntimeValue(10));
	assert_eq!(result.is_ok(), true);
	println!("#2 {:?}", TEST);
	
	let result = TEST.set_once(TestValue::RuntimeValue(20));
	assert_eq!(result.is_ok(), false);
	assert_eq!(*TEST, TestValue::RuntimeValue(10));
	println!("#3 {:?}", TEST);
	
	let result = TEST.replace_once(TestValue::Unk);
	assert_eq!(result, None);
	println!("#4 {:?}", result);
	
	
	//COPY
	let result = TEST.replace_once_copy(TestValue::Unk);
	assert_eq!(result.is_err(), true);
	
	println!("#5 result:{:?}", result);
	let value = match result {
		Ok(_a) => unreachable!(),
		Err(e) => e.into_inner(),
	};	
	println!("#5 value:{:?}", value);
}