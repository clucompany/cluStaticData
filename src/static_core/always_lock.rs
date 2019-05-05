

use crate::StaticOnce;

#[derive(Debug)]
pub enum AlwaysLockOnce {}


impl StaticOnce for AlwaysLockOnce {
	#[inline(always)]
	fn raw_lock_once<F: FnOnce()>(&self, f: F) {}
	
	#[inline(always)]
	fn is_init_state(&self) -> bool {
		true
	}
}

