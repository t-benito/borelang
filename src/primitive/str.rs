use std::collections::HashMap;
use crate::{Type, Value};
use crate::runtime::error::RuntimeErrorType;
pub fn register(vtables: &mut HashMap<Type, HashMap<String, Value>>) {
    let mut vtable = HashMap::new();
    vtable.insert("to_int".to_string(), Value::RustFunction {
        func: _toint,
        params: vec![],
        ret_type: Type::Int,
    });
    vtable.insert("to_float".to_string(), Value::RustFunction {
        func: _tofloat,
        params: vec![],
        ret_type: Type::Float,
    });
    vtable.insert("is_int".to_string(), Value::RustFunction {
        func: _isint,
        params: vec![],
        ret_type: Type::Bool,
    });
    vtable.insert("is_float".to_string(), Value::RustFunction {
        func: _isfloat,
        params: vec![],
        ret_type: Type::Bool,
    });
    vtables.insert(Type::Str, vtable);
}

pub fn _isint(args: Vec<Value>) -> Result<Value, RuntimeErrorType> {
    let this = &args[0];
    if let Value::Str(str) = this {
        return if let Ok(_) = str.parse::<i64>() {
            Ok(Value::Bool(true))
        } else {
            Ok(Value::Bool(false))
        }
    } unreachable!()
}
pub fn _isfloat(args: Vec<Value>) -> Result<Value, RuntimeErrorType> {
    let this = &args[0];
    if let Value::Str(str) = this {
        return if let Ok(_) = str.parse::<f64>() {
            Ok(Value::Bool(true))
        } else {
            Ok(Value::Bool(false))
        }
    } unreachable!()
}
pub fn _toint(args: Vec<Value>) -> Result<Value, RuntimeErrorType> {
    let this = &args[0];
    if let Value::Str(str) = this {
        return if let Ok(i) = str.parse::<i64>() {
            Ok(Value::Int(i))
        } else {
            Err(RuntimeErrorType::Other(format!("Failed to parse '{}' to an int", str)))
        }
    } unreachable!()
}
pub fn _tofloat(args: Vec<Value>) -> Result<Value, RuntimeErrorType> {
    let this = &args[0];
    if let Value::Str(str) = this {
        return if let Ok(f) = str.parse::<f64>() {
            Ok(Value::Float(f))
        } else {
            Err(RuntimeErrorType::Other(format!("Failed to parse '{}' to a float", str)))
        }
    } unreachable!()
}