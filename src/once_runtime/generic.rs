

pub trait InitializeRuntimeStaticData<'a, T> {
	fn initialize(&'a self) -> &'a T;
}

