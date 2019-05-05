
use crate::StaticOnce;
use std::sync::Once;

impl StaticOnce for Once {
	#[inline(always)]
	fn raw_lock_once<F: FnOnce()>(&self, f: F) {
		self.call_once(f)
	}
	
	
	#[inline(always)]
	fn is_init_state(&self) -> bool {
		self.is_completed()
	}
}