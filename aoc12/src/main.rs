//#![feature(custom_derive, plugin)]
//#![plugin(serde_macros)]

extern crate serde_json;
use serde_json::Value;

use std::io;

fn unroll_array(obj: &Value) -> i64 {
    let mut total = 0i64;

    for elem in obj.as_array().unwrap() {
        match elem {
            &Value::I64(i) => total += i,
            &Value::U64(u) => total += u as i64,
            &Value::F64(_) => panic!("got floating point value"),
            &Value::Object(_) => total += unroll_object(elem),
            &Value::Array(_) => total += unroll_array(elem),
            _ => (),
        }
    }
    total
}

fn unroll_object(obj: &Value) -> i64 {
    let mut total = 0i64;
    for (_, v) in obj.as_object().unwrap() {
        match v {
            &Value::I64(i) => total += i,
            &Value::U64(u) => total += u as i64,
            &Value::F64(_) => panic!("got floating point value"),
            &Value::String(ref s) => if s == "red" {return 0;},
            &Value::Object(_) => total += unroll_object(v),
            &Value::Array(_) => total += unroll_array(v),
            _ => (),
        }
    }
    total
}

fn main() {
    let data: Value = serde_json::from_reader(io::stdin()).unwrap();
    println!("{}", unroll_object(&data));
}

