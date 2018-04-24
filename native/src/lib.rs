#[macro_use]
extern crate neon;

use std::ops::DerefMut;
use neon::mem::Handle;
use neon::vm::{Call, JsResult};
use neon::js::{JsArray, Object, Value, JsValue};


fn cartesian_product_of(call: Call) -> JsResult<JsValue> {
    let scope = call.scope;
    let args = call.arguments;
    let args_len = args.len();
    let mut arguments = Vec::new();
    let mut accumulator: Vec<Vec<JsValue>> = Vec::new();
    accumulator.push(Vec::new());

    for index in 0..args_len {
        let a = args.get(scope, index).unwrap().check::<JsArray>()?;
        arguments.push(a);
    }

    let results = match arguments.split_first() {
        Some((first, rest)) => {
            
        },
        None => {
            vec![]
        }
    };

    Ok(finalArray.as_value(scope))
}

register_module!(m, {
    m.export("cartesianProductOf", cartesian_product_of)
});
