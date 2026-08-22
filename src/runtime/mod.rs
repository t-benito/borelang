pub mod error;
pub mod interpreter;
mod builtins;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use crate::{Type, Value};
use crate::runtime::error::RuntimeErrorType;
use crate::runtime::interpreter::Variable;

pub struct Module {
    pub obj: Value,
}
impl Module {
    pub fn new(name: &str) -> Self {
        Self {
            obj: Value::Object { name: name.to_string(), fields: Rc::new(RefCell::new(HashMap::new())) },
        }
    }
    pub fn add_fn(
        &mut self, 
        name: &str, 
        func: fn(Vec<Value>) -> Result<Value, RuntimeErrorType>,
        params: Vec<Type>,
        ret_type: Type, 
        is_mutable: bool
    ) -> &mut Self {
        if let Value::Object { fields, .. } = &self.obj {
            let value = Value::RustFunction {
                func,
                params,
                ret_type,
            };
            let ty = value.to_type();
            let mut borrow = fields.borrow_mut();
            borrow.insert(name.to_string(), Variable { 
                value,
                is_mutable,
                ty, 
            });
        }
        self
    }
    pub fn add_primitive(&mut self, name: &str, value: Value, is_mutable: bool) -> &mut Self {
        if let Value::Object { fields, .. } = &self.obj {
            let ty = value.to_type();
            let mut borrow = fields.borrow_mut();
            borrow.insert(name.to_string(), Variable {
                value,
                is_mutable,
                ty,
            });
        }
        self
    }
}