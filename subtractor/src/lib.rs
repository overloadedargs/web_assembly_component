#[allow(warnings)]
mod bindings;

use bindings::exports::docs::subtractor::subtract::Guest;

struct Component;

impl Guest for Component {
    fn subtract(a: i32, b: i32) -> i32 {
        return a - b;
    }
}

bindings::export!(Component with_types_in bindings);
