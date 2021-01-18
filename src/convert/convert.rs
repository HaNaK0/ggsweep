pub trait Convert<U> {
	fn convert_to(&self) -> U;
	fn convert_from(other: U) -> Self;
}