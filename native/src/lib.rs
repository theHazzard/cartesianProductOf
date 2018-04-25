#[macro_use]
extern crate neon;

use neon::mem::Handle;
use neon::scope::Scope;
use neon::vm::{Call, JsResult};
use neon::js::{JsArray, Object, JsValue};


fn cartesian_product_of(call: Call) -> JsResult<JsArray> {
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

    // argument
    // {
    //     JsArray([item1], [item2], [item3])
    // }
    // {
    //     JsArray([item4], [item5], [item6])
    // }
    // {
    //     JsArray([item7], [item8], [item9])
    // }

    match args_len {
        1 => return Ok(arguments[0]),
        _ => {
            // fisrt argument
            // {
            //     JsArray([item1], [item2], [item3])
            // }
            let mut acc: Vec<Handle<JsArray>> = Vec::new();
            let first_arg: Vec<Handle<JsValue>> = arguments
                .first()
                .expect("Cannot get first argument")
                .to_vec(scope)
                .expect("Cannot convert to vector");

            let rest = arguments[1..].to_vec();
            // rest
            // {
            //     JsArray([item4], [item5], [item6])
            // }
            // {
            //     JsArray([item7], [item8], [item9])
            // }
            for item in first_arg {
                let mut new_array = JsArray::new(scope, 1);
                new_array.set(1, item).expect("Cannot set value in JsArray");
                acc.push(new_array);
            }
            // acc
            // {
            //     JsArray([item1]),
            //     JsArray([item2]),
            //     JsArray([item3]),
            // }
            let (scope, ret_value) = cartesian(acc, rest, scope);
            let js_return = JsArray::new(&mut scope, ret_value.len() as u32);
            for (i, item) in ret_value.iter().enumerate() {
                js_return.set(i as u32, *item);
            }
            return Ok(js_return)
        },
    }

}

fn cartesian<'a, T: Scope<'a>>(acc: Vec<Handle<JsArray>>, rest: Vec<Handle<JsArray>>, scope: &'a mut T) -> (Scope<'a> , Vec<Handle<'a, JsArray>>) {
    // acc
    // {
    //     JsArray([item1]),
    //     JsArray([item2]),
    //     JsArray([item3]),
    // }
    // rest
    // {
    //     JsArray([item4], [item5], [item6])
    // }
    // {
    //     JsArray([item7], [item8], [item9])
    // }
    let popped_items = rest.first()
        .expect("Cannot get first argument")
        .to_vec(scope)
        .expect("Cannot convert to vector");
    // popped_items
    // {
    //     [[item4], [item5], [item6]]
    // }
    let rest = rest[1..].to_vec();
    // rest
    // {
    //     JsArray([item7], [item8], [item9])
    // }
    let acc_copy = acc.to_vec();
    // acc
    // {
    //     JsArray([item1]),
    //     JsArray([item2]),
    //     JsArray([item3]),
    // }
    let mut accumulator = Vec::new();
    for acc_item in acc_copy {
        let mut previous = acc_item.to_vec(scope).expect("Cannot convert to vector");
        // previous
        // {
        //     JsArray([item1]), <- THIS
        //     JsArray([item2]),
        //     JsArray([item3]),
        // }
        for popped_item in &popped_items {
            let previous = previous.to_vec();
            previous.push(*popped_item);
            let js_arr = JsArray::new(scope, previous.len() as u32);
            for (i, item) in previous.iter().enumerate() {
                js_arr.set(i as u32, *item);
            }
            accumulator.push(js_arr);
        }
        // previous
        // {
        //     JsArray([item1], [item4]), <- THIS
        //     JsArray([item2]),
        //     JsArray([item3]),
        // }

    }
    // expected
    // {
    //     JsArray([item1, item4]),
    //     JsArray([item1, item5]),
    //     JsArray([item1, item6]),
    //     JsArray([item2, item4]),
    //     JsArray([item2, item5]),
    //     JsArray([item2, item6]),
    //     JsArray([item3, item4]),
    //     JsArray([item3, item5]),
    //     JsArray([item3, item6]),
    // }
    (scope, accumulator)
}


// [
//
// ]

// [
// JsArray [
//     [1], <-
// ],
// JsArray [
//     [3],
// ],
// JsArray [
//     [5],
// ]
// ]

// [
//     JsArray [
//         [1],[3],[5]
//     ],
//     JsArray [
//         [],[],[]
//     ]
// ]



register_module!(m, {
    m.export("cartesianProductOf", cartesian_product_of)
});
