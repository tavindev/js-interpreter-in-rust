use dyn_clone::DynClone;

use crate::{interpreter::Interpreter, value::Value};

pub trait Callable: DynClone {
    fn name(&self) -> String;
    fn set_name(&mut self, name: String);
    fn call(&self, interpreter: &mut Interpreter, arguments: Vec<Value>) -> Value;
    fn arity(&self) -> usize;
}

dyn_clone::clone_trait_object!(Callable);

impl PartialEq for dyn Callable {
    fn eq(&self, other: &Self) -> bool {
        self.name() == other.name()
    }
}
