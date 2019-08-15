
use core::sync::atomic::AtomicU8;
use core::sync::atomic::Ordering;
use core::cell::UnsafeCell;

use crate::once_const_static::generic::UnsafeGenericStaticData;
use crate::once_const_static::generic::GenericStaticData;
use crate::once_const_static::UnkStaticData;

use crate::err::IgnoreInitErr;
use crate::err::StaticErr;


const UNINITIALIZED: u8 = 0;
//неинициализированным

const INITIALIZING: u8 = 1;
//инициализация

const INITIALIZED: u8 = 2;
//инициализирован


impl<T> UnkStaticData<T, AtomicU8> {
	#[inline]
	pub const fn new(a: T) -> Self {
		Self {
			data: UnsafeCell::new(a),
			sync_data: AtomicU8::new(UNINITIALIZED),
		}
	}
	
	#[inline]
	fn lock_logic<B: AtomicGenErr<VT, R>, A: FnOnce(VT) -> R, R, VT>(&self, v: VT, a: A) -> R {
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
				B::create_err(v)
			},
			_ => {
				B::create_err(v)
			},
			//инициализируется
		}
	}
	
	#[inline]
	fn ignore_initialize_logic<B: AtomicGenErr<(), R>, A: FnOnce() -> R, R>(&self, a: A) -> R {
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
				B::create_err(())
			},
			_ => B::create_err(()),
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
	fn raw_ignore_initialize_logic(&self) {
		//self.sync_data.store(EARLY_GET, Ordering::SeqCst);
		let _e = self.sync_data.compare_and_swap(UNINITIALIZED, INITIALIZING, Ordering::SeqCst);
	}
}


impl<T> UnsafeGenericStaticData<T> for UnkStaticData<&'static T, AtomicU8> where T: 'static {
	unsafe fn set_box(&self, v: Box<T>) -> Result<(), StaticErr<Box<T>>> {
		self.lock_logic::<StaticErrPrev, _, _, _>(v, |v| {
			#[allow(unused_unsafe)]
			unsafe {
				*self.data.get() = &*Box::into_raw(v);
			}
			Ok( () )
		}/*, |v| Err(StaticErr::prev(v))*/)
	}
	
	unsafe fn set_raw(&self, v: T) -> Result<(), StaticErr<T>> {
		self.lock_logic::<StaticErrPrev, _, _, _>(v, |v| {
			let v = Box::new(v);
			
			#[allow(unused_unsafe)]
			unsafe {
				*self.data.get() = &*Box::into_raw(v);
			}
			Ok( () )
		}/*, |v| Err(StaticErr::prev(v))*/)
	}
}

impl<T> GenericStaticData<T> for UnkStaticData<T, AtomicU8> {
	fn set(&self, v: T) -> Result<(), StaticErr<T>> {
		self.lock_logic::<StaticErrPrev, _, _, _>(v, 
			|v| { unsafe { *self.data.get() = v; } Ok( () )},
			//|v| Err(StaticErr::prev(v))
		)
	}
	
	fn replace(&self, v: T) -> Result<T, StaticErr<T>> {
		self.lock_logic::<StaticErrPrev, _, _, _>(v, 
			|v| Ok(	std::mem::replace(unsafe { &mut *self.data.get() }, v)	),
			//|v| Err(	StaticErr::prev(v)							),
			
		)
	}
	
	unsafe fn unsafe_replace(&self, v: T) -> T {
		#[allow(unused_unsafe)]
		std::mem::replace(unsafe { &mut *self.data.get() }, v)
	}
	
	fn get<'a>(&'a self) -> &'a T {
		unsafe{ &*self.data.get() }
	}
	
	fn ignore_initialize(&self) -> Result<(), IgnoreInitErr> {
		self.ignore_initialize_logic::<IgnoreInitErrPrev, _, _>( 
			|| Ok( () ),
			//|| Err( IgnoreInitErr::prev() )
		)
	}
	
	#[inline(always)]
	fn ignore_initialize_dont_result(&self) {
		self.raw_ignore_initialize_logic()
	}
	
	#[inline(always)]
	fn is_init_state(&self) -> bool {
		self.is_init_state_logic()
	}
}


trait AtomicGenErr<D, T> {
	fn create_err(d: D) -> T;
}


enum IgnoreInitErrPrev {}
impl<OKR, D> AtomicGenErr<D, Result<OKR, IgnoreInitErr>> for IgnoreInitErrPrev {
	#[inline(always)]
	fn create_err(_d: D) -> Result<OKR, IgnoreInitErr> {
		Err(IgnoreInitErr::prev())
	}
}


enum StaticErrPrev {}
impl<OKR, D> AtomicGenErr<D, Result<OKR, StaticErr<D>>> for StaticErrPrev {
	#[inline(always)]
	fn create_err(d: D) -> Result<OKR, StaticErr<D>> {
		Err(StaticErr::prev(d))
	}
}
