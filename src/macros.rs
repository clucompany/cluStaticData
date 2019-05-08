#[macro_export]
macro_rules! static_data {
	//static data
	[
		$(#[$($mt:tt)*])*
		static ref $name:ident: $t: ty = $a:expr;	$($tt:tt)*
	] => {
		$(#[$($mt)*])*
		static $name: $crate::StaticData<$t> = $crate::StaticData::new($a);
		
		$crate::static_data! {
			$($tt)*
		}
	};
	
	[
		$(#[$($mt:tt)*])*
		pub $(($($at:tt)*))* static ref $name:ident: $t: ty = $a:expr;	$($tt:tt)*
	] => {
		$(#[$($mt)*])*
		pub $(($($at)*))* static $name: $crate::StaticData<$t> = $crate::StaticData::new($a);

		
		$crate::static_data! {
			$($tt)*
		}
	};
	
	
	//lazy static data
	[
		$(#[$($mt:tt)*])*
		$(($($at:tt)*))* static ref +runtime $name:ident : $t: ty = $a:expr;	$($tt:tt)*
	] => {
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
		$(($($at)*))* static $name: $name::_HIDDEN = $name::_HIDDEN::new();
		
		
		$crate::static_data! {
			$($tt)*
		}
	};
	
	
	[
		$(#[$($mt:tt)*])*
		pub $(($($at:tt)*))* static ref +runtime $name:ident : $t: ty = $a:expr;	$($tt:tt)*
	] => {
		#[cfg(feature = "enable_runtime")]
		#[allow(non_snake_case)]
		#[doc(hidden)]
		mod $name {
			use super::*;
			$crate::new_lazy_struct! {
				$(#[$($mt)*])*
				pub(crate) _HIDDEN : $t = $a;
			}
		}
		
		#[cfg(feature = "enable_runtime")]
		$(#[$($mt)*])*
		pub $(($($at)*))* static $name: $name::_HIDDEN = $name::_HIDDEN::new();
		
		
		$crate::static_data! {
			$($tt)*
		}
	};
	
	() => ()
}

#[cfg(feature = "enable_runtime")]
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
		pub $(($($at)*))* struct $name($crate::RuntimeStaticData::Once<$t>);
		
		#[doc(hidden)]
		impl $name {
			#[inline]
			pub const fn new() -> Self {
				$name($crate::RuntimeStaticData::Once::INIT)
			}
			
			#[inline(always)]
			pub fn get<'a>(&'a self) -> &'a $t where Self: std::ops::Deref<Target = $t> {
				std::ops::Deref::deref(self)
			}
		}
		
		impl std::ops::Deref for $name {
			type Target = $t;
			
			fn deref(&self) -> &Self::Target {
				self.0.call_once(|| $b)
			}
		}
		
		impl std::convert::AsRef<$t> for $name {
			#[inline(always)]
			fn as_ref(&self) -> &$t {
				std::ops::Deref::deref(self)
			}
		}
		
		impl std::fmt::Debug for $name 
			where <Self as std::ops::Deref>::Target:	std::fmt::Debug 
			{
				
			#[inline(always)]
			fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
				std::fmt::Debug::fmt(std::ops::Deref::deref(self), f)
			}
		}
		
		#[cfg(feature = "trivial_bounds")]
		impl std::fmt::Display for $name 
			where <Self as std::ops::Deref>::Target:	std::fmt::Display 
			{
			
			#[inline(always)]
			fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
				std::fmt::Display::fmt(std::ops::Deref::deref(self), f)
			}
		}
	};
}