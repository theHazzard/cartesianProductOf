#[macro_use]
extern crate neon;

use neon::mem::{Handle};
use neon::scope::Scope;
use neon::vm::{Call, JsResult};
use neon::js::{JsString, JsArray, Object, JsObject};

fn jsarray_as_vector<'a, T: Scope<'a>> (scope: &mut T, jarray: &JsArray) -> Vec<JsObject> {
    let vector: Vec<Handle<JsObject>> = Vec::new();

    for elem in 0..jarray.len() {
        let newElem: JsObject = jarray.get(scope, elem).unwrap().deref();
        vector.push(newElem);
    }
    vector
}

fn cartesian_product_of(call: Call) -> JsResult<JsString> {
    let scope = call.scope;
    let raw_args = call.arguments;
    let args_len = raw_args.len();

    let mut result:Vec<Vec<JsArray>> = Vec::new();
    result.push(Vec::new());

    for i in 0..args_len {
        let mut result: Vec<Vec<JsArray>> = Vec::new();
        for _elem in result.iter() {
            let arg_elements: Handle<JsArray> = raw_args.get(scope, i).unwrap();

            for l in 0..arg_elements.len() {
                let item_elem = arg_elements.get(scope, l).unwrap().check::<JsArray>().unwrap();

                let arr = jsarray_as_vector(scope, &item_elem);
            }
        }
    }
    Ok(JsString::new(scope, "Hola").unwrap())
}

register_module!(m, {
    m.export("cartesianProductOf", cartesian_product_of)
});
