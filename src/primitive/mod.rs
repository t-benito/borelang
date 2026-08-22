use std::collections::HashMap;
use crate::{Type, Value};

mod int;
mod float;
mod str;

pub fn register(vtables: &mut HashMap<Type, HashMap<String, Value>>) {
    int::register(vtables);
    float::register(vtables);
    str::register(vtables);
}