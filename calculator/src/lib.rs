#[allow(warnings)]
mod bindings;

use bindings::exports::docs::calculator::calculate::{Guest, Op};

// Bring the imported add function into scope
use bindings::docs::adder::add::add;
use bindings::docs::subtractor::subtract::subtract;

struct Component;

impl Guest for Component {
    // fn eval_expression(expr: String) -> u32 {
    //     // Cleverly parse `expr` into values and operations, and evaluate
    //     // them meticulously.
    //     add(123, 456)
    // }
    fn eval_expression(op: Op, x: i32, y: i32) -> i32 {
        match op {
            Op::Add => add(x, y),
            Op::Subtract => subtract(x, y),
        }
    }
}

bindings::export!(Component with_types_in bindings);
