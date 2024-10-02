mod bindings;

use clap::Parser;
use std::fmt;

use bindings::docs::calculator::{calculate, calculate::Op};

fn parse_operator(op: &str) -> anyhow::Result<Op> {
    match op {
        "add" => Ok(Op::Add),
        "subtract" => Ok(Op::Subtract),
        "interest_rate" => Ok(Op::InterestRate),
        _ => anyhow::bail!("Unknown operation: {}", op),
    }
}

impl fmt::Display for Op {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Op::Add => write!(f, "+"),
            Op::Subtract => write!(f, "-"),
            Op::InterestRate => write!(f, "InterestRate")
        }
    }
}

/// A CLI for executing mathematical expressions
/// using WebAssembly
#[derive(Parser)]
#[clap(name = "calculator", version = env!("CARGO_PKG_VERSION"))]
struct Args {
    /// Expression operator
    #[clap(value_parser = parse_operator)]
    op: Op,
    /// The first operand
    x: f32,
    /// The second operand
    y: f32,
    /// The third operand
    z: Option<f32>,
}

// #[derive(Subcommand)]
// enum Commands {
//     /// does testing things
//     Test {
//         /// lists test values
//         #[arg(short, long)]
//         list: bool,
//     },
// }   

fn main() {
    let cli = Args::parse();
    // You can check the value provided by positional arguments, or option arguments
    if let Some(z) = cli.z {
        let res = calculate::eval_expression_three_args(cli.op, cli.x, cli.y as u32, z as u32);
        println!("{}", res);
        println!("{} {} {} {} = {res}", cli.x, cli.op, cli.y, z);
    } else {
        let res = calculate::eval_expression(cli.op, cli.x as f32, cli.y as f32);
        println!("{} {} {} = {res}", cli.x, cli.op, cli.y);
    }
}