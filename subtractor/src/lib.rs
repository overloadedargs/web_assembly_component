#[allow(warnings)]
mod bindings;

use bindings::exports::docs::subtractor::subtract::Guest;

struct Component;

impl Guest for Component {
    fn subtract(a: f32, b: f32) -> f32 {
        return a - b;
    }
}

bindings::export!(Component with_types_in bindings);
