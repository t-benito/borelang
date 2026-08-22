use std::io::{stdout, BufWriter, Stdout, Write, stdin};
use std::rc::Rc;
use std::sync::Mutex;

use crate::Module;
use crate::runtime::error::RuntimeErrorType;
use crate::runtime::interpreter::{Value, Type};

pub fn init_module() -> Module {
    let mut module = Module::new("builtins");
    module.add_fn(
        "println",
        _println,
        vec![Type::Any],
        Type::Null,
        false
    );
    module.add_fn(
        "print",
        _print,
        vec![Type::Any],
        Type::Null,
        false
    );
    module.add_fn(
        "input",
        _read,
        vec![Type::Str],
        Type::Str,
        false
    );
    module.add_primitive(
        "null",
        Value::Null,
        false
    );
    module.add_primitive(
        "true",
        Value::Bool(true),
        false
    );
    module.add_primitive(
        "false",
        Value::Bool(false),
        false
    );
    module
}
pub fn _println(args: Vec<Value>) -> Result<Value, RuntimeErrorType> {
    let mut lock = __stdout().lock().unwrap();
    for arg in args {
        write!(lock, "{}", arg).unwrap();
    }
    writeln!(lock).unwrap();
    lock.flush().unwrap();
    Ok(Value::Null)
}
pub fn _print(args: Vec<Value>) -> Result<Value, RuntimeErrorType> {
    let mut lock = __stdout().lock().unwrap();
    for arg in args {
        match arg {
            Value::Bool(b) => write!(lock, "{}", b).unwrap(),
            Value::Int(i) => write!(lock, "{}", i).unwrap(),
            Value::Float(f) => write!(lock, "{}", f).unwrap(),
            Value::Str(s) => write!(lock, "{}", s).unwrap(),
            other => write!(lock, "{:?}", other).unwrap(),
        }
    }
    lock.flush().unwrap();
    Ok(Value::Null)
}
pub fn _read(args: Vec<Value>) -> Result<Value, RuntimeErrorType> {
    if let Value::Str(str) = &args[0] {
        let mut s = String::new();
        let mut lock = __stdout().lock().unwrap();
        write!(lock, "{}", str).unwrap();
        lock.flush().unwrap();
        stdin().read_line(&mut s).expect("");
        return Ok(Value::Str(Rc::new(s.trim().to_string())));
    } unreachable!()
}

fn __stdout() -> &'static Mutex<BufWriter<Stdout>> {
    static WRITER: std::sync::OnceLock<Mutex<BufWriter<Stdout>>> = std::sync::OnceLock::new();
    WRITER.get_or_init(|| Mutex::new(BufWriter::new(stdout())))
}