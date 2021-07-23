use std::marker::PhantomData;

/// See [`rant::Rant`].
pub struct Rant;
pub struct RantProgram;
pub struct RantValue;
pub struct RuntimeResult<T>(PhantomData<T>);
