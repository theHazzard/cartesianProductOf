#[macro_use]
extern crate neon;

use neon::mem::Handle;
use neon::ops::Try;
use neon::vm::{Call, JsResult};
use neon::js::{JsArray, Object, JsValue, JsString};


fn cartesian_product_of(call: Call) -> JsResult<JsString> {
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
    let mut arr: Vec<Vec<Handle<_>>> = Vec::new();
    arguments.iter().fold(arr, | acc, &array | {
        let mut local_acc: Vec<Vec<Handle<JsValue>>> = Vec::new();

        for acc_item in local_acc {
            let mut s = 0;
            for a in 0..array.len() {
                let elem1 = array.get(scope, a).unwrap().check::<JsArray>().unwrap().to_vec(scope);
                acc_item.append(&mut elem1.unwrap());
                let jsarray = JsArray::new(scope, acc_item.len() as u32).check::<JsArray>().unwrap();
                let jsarray = acc_item.into_iter().enumerate().fold(jsarray, |acc, (i, elem)| {
                    acc.set(i as u32, elem);
                    acc
                });
                local_acc.push(acc_item);
                println!("{}", 'a');
            }
        }
        local_acc
    });

    Ok(JsString::new(scope, "hola").unwrap())
}

register_module!(m, {
    m.export("cartesianProductOf", cartesian_product_of)
});
