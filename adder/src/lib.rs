#[allow(warnings)]
mod bindings;

use bindings::exports::docs::adder::add::Guest;

struct Component;

impl Guest for Component {
    fn add(a: i32, b: i32) -> i32 {
        return a + b;
    }
}

bindings::export!(Component with_types_in bindings);
