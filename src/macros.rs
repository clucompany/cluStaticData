
#[macro_export]
macro_rules! static_data {
	[
		$(#[$($mt:tt)*])* pub $(($($v_t:tt)*))* static ref $name:ident: $t: ty = $a:expr; $($tt:tt)*
	] => {
		$crate::once_const_static_data! {
			$(#[$($mt)*])* pub $( ($($v_t)*) )* static ref $name : $t = $a;
		}
		
		$crate::static_data! {
			$($tt)*
		}
	};
	[
		$(#[$($mt:tt)*])* static ref $name:ident: $t: ty = $a:expr; $($tt:tt)*
	] => {
		$crate::once_const_static_data! {
			$(#[$($mt)*])* static ref $name : $t = $a;
		}
		
		$crate::static_data! {
			$($tt)*
		}
	};
	//END DEF.
	
	//RUNTIME
	[
		$(#[$($mt:tt)*])* pub $(($($v_t:tt)*))* static ref +runtime $name:ident: $t: ty = $a:expr; $($tt:tt)*
	] => {
		#[cfg(feature = "once_runtime")]
		$crate::once_runtime_data! {
			$(#[$($mt)*])* pub $( ($($v_t)*) )* static ref +runtime $name : $t = $a;
		}
		
		#[cfg(not(feature = "once_runtime"))]
		compile_error!("Expected flag `feature=once_runtime` and `feature=trivial_bounds`");
		
		$crate::static_data! {
			$($tt)*
		}
	};
	[
		$(#[$($mt:tt)*])* static ref +runtime $name:ident: $t: ty = $a:expr; $($tt:tt)*
	] => {
		#[cfg(feature = "once_runtime")]
		$crate::once_runtime_data! {
			$(#[$($mt)*])* static ref +runtime $name : $t = $a;
		}
		
		#[cfg(not(feature = "once_runtime"))]
		compile_error!("Expected flag `feature=once_runtime` and `feature=trivial_bounds`");
		
		$crate::static_data! {
			$($tt)*
		}
	};
	

	() => ()
}
