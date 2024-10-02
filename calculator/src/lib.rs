#[allow(warnings)]
mod bindings;

use bindings::exports::docs::calculator::calculate::{Guest, Op};

// Bring the imported functions into scope
use bindings::docs::adder::add::add;
use bindings::docs::subtractor::subtract::subtract;

struct Component;

impl Guest for Component {

    fn eval_expression(op: Op, x: f32, y: f32) -> f32 {
        match op {
            Op::Add => add(x, y),
            Op::Subtract => subtract(x, y),
            Op::InterestRate => 0.0 as f32 // throw error  
        }
    }

    fn eval_expression_three_args(op: Op, x: f32, y: u32, z: u32) -> f32 {
        match op {
            Op::Add => 0.0 as f32,
            Op::Subtract => 0.0 as f32,
            Op::InterestRate => Component::total_payable(x, y, z)
        }
    }

    // fn eval_expression(expr: String) -> u32 {
    //     // Cleverly parse `expr` into values and operations, and evaluate
    //     // them meticulously.
    //     add(123, 456)
    // }

    // M = payment amount
    // P = principal, meaning the amount of money borrowed
    // J = effective interest rate. Note that this is usually not the annual interest rate; see below for an explanation.
    // N = total number of payments
    // M = P * ( J / (1 - (1 + J)-N)).
    
    
    // Taking into account amount repayed
    // a: The loan is for $200,000,
    // r: 0.01 (12% annual rate, or 0.12, divided by 12 monthly payments yearly),
    // n: 360 (12 monthly payments per year times 30 years)
    // P = a / { [ ( 1+r ) ^ n ] -1} / [r * (1+r) ^n]
    // 200,000 ÷ { [ ( 1 + 0.01) ^ 360 ] – 1 } ÷ [ 0.01 ( 1 + 0.01 ) ^ 360 ] = $2005.51
    // or
    // Periodic loan payment = Loan amount / (((1 + Periodic rate) ^ Number of payments) - 1) / (Periodic rate × ((1 + Periodic rate) ^ Number of payments)).
    //
    fn total_payable(rate: f32, amount: u32, years: u32) -> f32 {
        let full_amount = amount as f32;
        let interest_rate = rate as f32 / 12.0;
        let payments = years as f32 * 12.0;

        let mut to_repay = full_amount / ((subtract((add(1.0, interest_rate) as f32).powf(payments), 1.0)) / (interest_rate * add(1.0, interest_rate).powf(payments)));
  
        return to_repay as f32;
      }
}

bindings::export!(Component with_types_in bindings);
