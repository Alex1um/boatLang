use crate::boat_program::{BoatExpr, BoatOp, Function, Functions};
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

pub fn translate_expr(expr: BoatExpr, function_map: &Functions) -> Vec<BoatIns> {
    match expr {
        BoatExpr::Value(value) => vec![BoatIns::Push { value }],
        BoatExpr::Var(name) => vec![BoatIns::KVGet { key: name }],
        BoatExpr::Function { name, args } => {
            let function = function_map.get(&name).expect("Function is found");
            match function {
                Function::Predefined { instructions } => {
                    let mut translated: Vec<BoatIns> = args.into_iter().flat_map(|arg| translate_expr(arg, function_map)).collect();
                    translated.extend(instructions.clone());
                    translated
                },
                _ => unimplemented!("That type of function is unimplemented")
            }
        },
        BoatExpr::BinOp { lhs, op, rhs } => {
            let lhs_instructions = translate_expr(*lhs, function_map);
            let rhs_instructions = translate_expr(*rhs, function_map);
            let mut instructions = Vec::with_capacity(lhs_instructions.len() + rhs_instructions.len() + 1);
            instructions.extend(rhs_instructions);
            instructions.extend(lhs_instructions);
            instructions.push(op.into());
            instructions
        },
    }
}
