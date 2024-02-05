use std::{collections::HashMap, fmt::Display};

use crate::expr_parser::{BoatExpr, BoatOp};
use crate::program_parser::FunctionDefinition;


#[derive(Debug)]
pub enum BoatIns {
    Push { value: String }, // Push value on top of stack
    Pop, // Pop value from top of stack
    Goto { ins: u32 }, // Go to instruction with index
    Clone, // Clone value on top of stack
    Input { pin: u32 }, // Block until input value from pin and push it on top of stack
    Output{ pin: u32 }, // Pop value from top of stack and output it to pin
    Add, // Pop two values from top of stack and push their sum
    Sub, // Pop two values from top of stack and push their difference
    Mul, // Pop two values from top of stack and push their product
    Div, // Pop two values from top of stack and push their quotient
    Conc, // Pop two values from top of stack and push their concatenation
    KVSet { key: String }, // Pop value from top of stack and set key to that value on key-value storage
    KVGet { key: String }, // Push value from key-value storage to top of stack
    KVDel { key: String }, // Delete value by key from key-value storage
    Sleep { seconds: String }, // sleep for passed amount of time
    Eq, // Pop two values from top of stack and push 1 if they are equal or 0
    Gt, // Pop two values from top of stack and push 1 if the first is greater than the second or 0
}

impl Display for BoatIns {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use BoatIns::*;
        match self {
            Push { value } => write!(f, "p {};", value),
            Pop => write!(f, "d;"),
            Goto { ins } => write!(f, "g {};", ins),
            Clone => write!(f, "r;"),
            Input { pin } => write!(f, "i {};", pin),
            Output { pin } => write!(f, "o {};", pin),
            Add => write!(f, "+;"),
            Sub => write!(f, "-;"),
            Mul => write!(f, "*;"),
            Div => write!(f, "/;"),
            Conc => write!(f, "..;"),
            KVSet { key } => write!(f, "ks {};", key),
            KVGet { key } => write!(f, "kg {};", key),
            KVDel { key } => write!(f, "kd {};", key),
            Sleep { seconds } => write!(f, "s {};", seconds),
            Eq => write!(f, "=;"),
            Gt => write!(f, ">;"),
        }
    }
}

fn translate_op(op: BoatOp) -> BoatIns {
    use BoatOp::*;
    match op {
        Add => BoatIns::Add,
        Sub => BoatIns::Sub,
        Mul => BoatIns::Mul,
        Div => BoatIns::Div,
        Conc => BoatIns::Conc,
    }
}

pub fn translate_expr(expr: BoatExpr, function_map: &HashMap<String, FunctionDefinition>) -> Vec<BoatIns> {
    match expr {
        BoatExpr::Value(value) => vec![BoatIns::Push { value }],
        BoatExpr::Var(name) => vec![BoatIns::KVGet { key: name }],
        BoatExpr::Function { name, args } => {
            let function_def = function_map.get(&name).expect("Function is found");
            let mut instructions: Vec<BoatIns> = vec![BoatIns::Goto { ins: function_def.place }];
            for (arg_expr, arg_name) in args.into_iter().zip(function_def.args.iter()) {
                instructions.extend(translate_expr(arg_expr, function_map));
                instructions.push(BoatIns::KVSet { key: arg_name.clone() });
            }
            instructions
        },
        BoatExpr::BinOp { lhs, op, rhs } => {
            let lhs_instructions = translate_expr(*lhs, function_map);
            let rhs_instructions = translate_expr(*rhs, function_map);
            let mut instructions = Vec::with_capacity(lhs_instructions.len() + rhs_instructions.len() + 1);
            instructions.extend(lhs_instructions);
            instructions.extend(rhs_instructions);
            instructions.push(translate_op(op));
            instructions
        },
    }
}

pub fn translated_to_string(inses: Vec<BoatIns>) -> String {
    inses
        .into_iter()
        .map(|ins| format!("{}", ins))
        .collect::<Vec<String>>()
        .join("")
}
