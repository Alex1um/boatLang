use std::{collections::HashMap};

use crate::expr_parser::{BoatExpr, BoatOp};
use crate::program_parser::FunctionDefinition;
use crate::boat_instructions::BoatIns;

impl Into<BoatIns> for BoatOp {
    fn into(self) -> BoatIns {
        use BoatOp::*;
        match self {
            Add => BoatIns::Add,
            Sub => BoatIns::Sub,
            Mul => BoatIns::Mul,
            Div => BoatIns::Div,
            Conc => BoatIns::Conc,
            Gt => BoatIns::Gt,
            Eq => BoatIns::Eq,
        }
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
            instructions.push(op.into());
            instructions
        },
    }
}
