#[macro_export]
macro_rules! static_data {
	//static data
	[
		$(#[$($mt:tt)*])*
		static ref $name:ident: $t: ty = $a:expr;	$($tt:tt)*
	] => {
		$(#[$($mt)*])*
		static $name: $crate::StaticData<$t> = $crate::StaticData::new($a);
		
		static_data! {
			$($tt)*
		}
	};
	
	[
		$(#[$($mt:tt)*])*
		pub $(($($at:tt)*))* static ref $name:ident: $t: ty = $a:expr;	$($tt:tt)*
	] => {
		$(#[$($mt)*])*
		pub $(($($at)*))* static $name: $crate::StaticData<$t> = $crate::StaticData::new($a);

		
		static_data! {
			$($tt)*
		}
	};
	
	() => ()
}