use std::any::Any;

pub fn force_downcast<T: 'static>(b: Box<dyn Any + Send + Sync>) -> T {
    *<Box<dyn Any + Send>>::downcast::<T>(b).unwrap()
}
