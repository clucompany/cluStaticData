

#[macro_export]
macro_rules! once_const_static_data {
	[$(#[$($mt:tt)*])* static ref $name:ident: $t: ty = $a:expr; $($tt:tt)*] => {
		$(#[$($mt)*])*
		static $name: $crate::once_const_static::StaticData<$t> = $crate::once_const_static::StaticData::new($a);
		
		$crate::once_const_static_data! {
			$($tt)*
		}
	};
	
	[$(#[$($mt:tt)*])* pub $(($($at:tt)*))* static ref $name:ident: $t: ty = $a:expr; $($tt:tt)*] => {
		$(#[$($mt)*])*
		pub $(($($at)*))* static $name: $crate::once_const_static::StaticData<$t> = $crate::once_const_static::StaticData::new($a);
		
		$crate::once_const_static_data! {
			$($tt)*
		}
	};
	() => ()
}

