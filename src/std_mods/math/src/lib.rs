use borelang::{Type, Value, Module};
use borelang::runtime::error::RuntimeErrorType;

#[unsafe(no_mangle)]
pub extern "C" fn init_module() -> Module {
    let mut module = Module::new("math");
    module.add_fn(
        "abs", _abs,
        vec![
            Type::OneOf(vec![Type::Int, Type::Float])
        ],
        Type::OneOf(vec![Type::Int, Type::Float]),
        false
    );
    module.add_fn(
        "floor", _floor,
        vec![Type::Float],
        Type::Float,
        false
    );
    module.add_fn(
        "ceil", _ceil,
        vec![Type::Float],
        Type::Float,
        false
    );
    module.add_fn(
        "sin", _sin,
        vec![Type::OneOf(vec![Type::Int, Type::Float])],
        Type::Float,
        false
    );
    module.add_fn(
        "cos", _cos,
        vec![Type::OneOf(vec![Type::Int, Type::Float])],
        Type::Float,
        false
    );

    module.add_primitive(
        "PI", Value::Float(std::f64::consts::PI),
        false
    );
    module.add_primitive(
        "EULER", Value::Float(std::f64::consts::E),
        false
    );

    module
}
pub fn _abs(args: Vec<Value>) -> Result<Value, RuntimeErrorType> {
    match args[0] {
        Value::Float(x) => Ok(Value::Float(x.abs())),
        Value::Int(x) => Ok(Value::Int(x.abs())),
        _ => unreachable!()
    }
}
pub fn _floor(args: Vec<Value>) -> Result<Value, RuntimeErrorType>  {
    match args[0] {
        Value::Float(x) => Ok(Value::Float(x.floor())),
        _ => unreachable!()
    }
}
pub fn _ceil(args: Vec<Value>) -> Result<Value, RuntimeErrorType>  {
    match args[0] {
        Value::Float(x) => Ok(Value::Float(x.ceil())),
        _ => unreachable!()
    }
}
pub fn _sin(args: Vec<Value>) -> Result<Value, RuntimeErrorType>  {
    match args[0] {
        Value::Float(x) => Ok(Value::Float(x.sin())),
        Value::Int(x) => Ok(Value::Float((x as f64).sin())),
        _ => unreachable!()
    }
}
pub fn _cos(args: Vec<Value>) -> Result<Value, RuntimeErrorType>  {
    match args[0] {
        Value::Float(x) => Ok(Value::Float(x.cos())),
        Value::Int(x) => Ok(Value::Float((x as f64).cos())),
        _ => unreachable!()
    }
}