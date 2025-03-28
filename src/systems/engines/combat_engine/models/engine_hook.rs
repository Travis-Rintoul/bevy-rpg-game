use std::any::Any;

pub trait EngineHook: Any {
    fn as_any(&self) -> &dyn Any;
}

impl<T: Any> EngineHook for T {
    fn as_any(&self) -> &dyn Any {
        self
    }
}