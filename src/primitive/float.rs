use std::collections::HashMap;
use std::rc::Rc;
use crate::runtime::{ interpreter::{Type, Value} };
use crate::runtime::error::RuntimeErrorType;

pub fn register(vtables: &mut HashMap<Type, HashMap<String, Value>>) {
    let mut vtable = HashMap::new();
    vtable.insert("to_str".to_string(), Value::RustFunction {
        func: _tostr,
        params: vec![],
        ret_type: Type::Str,
    });
    vtables.insert(Type::Float, vtable);
}
fn _tostr(args: Vec<Value>) -> Result<Value, RuntimeErrorType> {
    let this = &args[0];
    if let Value::Int(v) = this {
        Ok(Value::Str(Rc::new(format!("{}", v))))
    } else { unreachable!() }
}
