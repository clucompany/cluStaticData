


#[doc(hidden)]
#[macro_export]
macro_rules! once_runtime_data {
	[$(#[$($mt:tt)*])* static ref +runtime $name:ident: $t: ty = $a:expr; $($tt:tt)*] => {
		#[allow(non_snake_case)]
		#[doc(hidden)]
		mod $name {
			use super::*;
			$crate::new_lazy_struct! {
				$(#[$($mt)*])*
				pub(crate) _HIDDEN : $t = $a;
			}
		}
		
		$(#[$($mt)*])*
		static $name: $name::_HIDDEN = $name::_HIDDEN::new();
		
		$crate::once_runtime_data! {
			$($tt)*
		}
	};
	
	[$(#[$($mt:tt)*])* pub $(($($at:tt)*))* static ref +runtime $name:ident: $t: ty = $a:expr; $($tt:tt)*] => {
		#[allow(non_snake_case)]
		#[doc(hidden)]
		mod $name {
			use super::*;
			$crate::new_lazy_struct! {
				$(#[$($mt)*])*
				pub(crate) _HIDDEN : $t = $a;
			}
		}
		
		$(#[$($mt)*])*
		pub $(($($at)*))* static $name: $name::_HIDDEN = $name::_HIDDEN::new();
		
		$crate::once_runtime_data! {
			$($tt)*
		}
	};
	
	() => ()
}



#[doc(hidden)]
#[macro_export]
macro_rules! new_lazy_struct {
	[
		$(#[$($mt:tt)*])*
		pub $(($($at:tt)*))*  $name:ident : $t: ty = $b:expr;
	] => {
		#[doc(hidden)]
		#[allow(missing_copy_implementations)]
		#[allow(non_camel_case_types)]
		#[allow(dead_code)]
		$(#[$($mt)*])*
		pub $(($($at)*))* struct $name($crate::once_runtime::Once<$t>);

		impl $name {
			#[doc(hidden)]
			#[inline]
			pub const fn new() -> Self {
				$name($crate::once_runtime::Once::INIT)
			}
			
			#[inline(always)]
			pub fn get<'a>(&'a self) -> &'a $t {
				&**self
			}
		}
		
		
		impl<'a> $crate::once_runtime::generic::InitializeRuntimeStaticData<'a, $t> for $name {
			fn initialize(&'a self) -> &'a $t {
				#[inline(always)]
				fn closure() -> $t {
					$b
				}
				
				self.0.call_once(closure)
			}
		}
		
		impl core::ops::Deref for $name {
			type Target = $t;
			
			#[inline(always)]
			fn deref(&self) -> &Self::Target {
				<$crate::once_runtime::generic::InitializeRuntimeStaticData<$t>>::initialize(self)
			}
		}
		
		impl core::convert::AsRef<$t> for $name {
			#[inline(always)]
			fn as_ref(&self) -> &$t {
				self.get()
			}
		}
		
		impl core::fmt::Debug for $name 
			where <Self as core::ops::Deref>::Target:	core::fmt::Debug 
			{
				
			#[inline(always)]
			fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
				core::fmt::Debug::fmt(self.get(), f)
			}
		}
		
		//#[cfg(feature = "trivial_bounds")]
		impl core::fmt::Display for $name 
			where <Self as core::ops::Deref>::Target:	core::fmt::Display 
			{
			
			#[inline(always)]
			fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
				core::fmt::Display::fmt(self.get(), f)
			}
		}
	};
}