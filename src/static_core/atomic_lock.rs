
use crate::err::IgnoreInitErr;
use crate::SetInitUnkStaticData;
use crate::UnsafeInitUnkStaticData;
use crate::err::StaticErr;
use crate::UnkStaticData;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;
use std::cell::UnsafeCell;

const UNINITIALIZED: usize = 0;
//неинициализированным

const INITIALIZING: usize = 1;
//инициализация

const EARLY_GET: usize = 3;

const INITIALIZED: usize = 2;
//инициализирован


impl<T> UnkStaticData<T, AtomicUsize> {
	#[inline]
	pub const fn new(a: T) -> Self {
		Self {
			data: UnsafeCell::new(a),
			sync_data: AtomicUsize::new(UNINITIALIZED),
		}
	}
	
	#[inline]
	fn lock_logic<A: FnOnce(VT) -> R, B: Fn(VT) -> R, R, VT>(&self, v: VT, a: A, b: B) -> R {
		match self.sync_data.compare_and_swap(UNINITIALIZED, INITIALIZING, Ordering::SeqCst) {
			UNINITIALIZED => {
				//неинициализированным
				let result = a(v);
				self.sync_data.store(INITIALIZED, Ordering::SeqCst);

				result
			},
			INITIALIZING => {
				//инициализация
				while self.sync_data.load(Ordering::SeqCst) == INITIALIZING {}
				b(v)
			},
			_ => {
				b(v)
			},
			//инициализируется
		}
	}
	
	#[inline]
	fn ignore_init_logic<A: FnOnce() -> R, B: Fn() -> R, R>(&self, a: A, b: B) -> R {
		match self.sync_data.compare_and_swap(UNINITIALIZED, INITIALIZING, Ordering::SeqCst) {
			UNINITIALIZED => {
				//неинициализированным
				let result = a();
				self.sync_data.store(INITIALIZED, Ordering::SeqCst);
				
				result
			},
			INITIALIZING => {
				//инициализация
				while self.sync_data.load(Ordering::SeqCst) == INITIALIZING {}
				b()
			},
			_ => b(),
			//инициализируется
		}
	}
	
	#[inline]
	fn is_init_state_logic(&self) -> bool {
		match self.sync_data.load(Ordering::SeqCst) {
			UNINITIALIZED => false,
			//неинициализированным
			
			_ => true,
		}
	}
	
	#[inline]
	fn raw_ignore_init_logic(&self) {
		self.sync_data.store(EARLY_GET, Ordering::SeqCst);
	}
}


impl<T> UnsafeInitUnkStaticData<T> for UnkStaticData<&'static T, AtomicUsize> where T: 'static {
	unsafe fn set_box(&self, v: Box<T>) -> Result<(), StaticErr<Box<T>>> {
		self.lock_logic(v, |v| {
			#[allow(unused_unsafe)]
			unsafe {
				*self.data.get() = &*Box::into_raw(v);
			}
			Ok( () )
		}, |v| Err(StaticErr::prev(v)))
	}
	
	unsafe fn set_raw(&self, v: T) -> Result<(), StaticErr<T>> {
		self.lock_logic(v, |v| {
			let v = Box::new(v);
			
			#[allow(unused_unsafe)]
			unsafe {
				*self.data.get() = &*Box::into_raw(v);
			}
			Ok( () )
		}, |v| Err(StaticErr::prev(v)))
	}
}

impl<T> SetInitUnkStaticData<T> for UnkStaticData<T, AtomicUsize> {
	fn set(&self, v: T) -> Result<(), StaticErr<T>> {
		self.lock_logic(v, 
			|v| { unsafe { *self.data.get() = v; } Ok( () )},
			|v| Err(StaticErr::prev(v))
		)
	}
	
	fn replace(&self, v: T) -> Result<T, StaticErr<T>> {
		self.lock_logic(v, 
			|v| Ok(	std::mem::replace(unsafe { &mut *self.data.get() }, v)	),
			|v| Err(	StaticErr::prev(v)							),
		)
	}
	
	unsafe fn unsafe_replace(&self, v: T) -> T {
		#[allow(unused_unsafe)]
		std::mem::replace(unsafe { &mut *self.data.get() }, v)
	}
	
	fn get<'a>(&'a self) -> &'a T {
		unsafe{ &*self.data.get() }
	}
	
	fn ignore_init(&self) -> Result<(), IgnoreInitErr> {
		self.ignore_init_logic( 
			|| Ok( () ),
			|| Err( IgnoreInitErr::prev() )
		)
	}
	
	#[inline(always)]
	fn ignore_init_dont_result(&self) {
		self.raw_ignore_init_logic()
	}
	
	#[inline(always)]
	fn is_init_state(&self) -> bool {
		self.is_init_state_logic()
	}
}