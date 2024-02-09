use crate::boat_program::{BoatExpr, BoatOp, Function, Functions};
use crate::boat_instructions::{BoatIns, BoatArg, BoatCmd};

impl Into<BoatCmd> for BoatOp {
    fn into(self) -> BoatCmd {
        use BoatOp::*;
        match self {
            Add => BoatCmd::Add,
            Sub => BoatCmd::Sub,
            Mul => BoatCmd::Mul,
            Div => BoatCmd::Div,
            Conc => BoatCmd::Conc,
            Gt => BoatCmd::Gt,
            Eq => BoatCmd::Eq,
        }
    }
}

pub fn translate_expr(arg: BoatExpr, instructions: &mut Vec<BoatIns>, functions: &Functions) -> BoatArg {
    match arg {
        BoatExpr::Value(value) => BoatArg::Const(value),
        BoatExpr::Var(name) => BoatArg::FromKVS(name),
        BoatExpr::Function { name, mut args } => {
            let mut instructions = Vec::<BoatIns>::new();
            let function = functions.get(&name).expect("Function is defined");
            let mut translated_args = Vec::<BoatArg>::new();
            args.reverse();
            for arg in args {
                translated_args.push(translate_expr(arg, &mut instructions, functions))
            }
            match function {
                Function::Predefined { translator } => {
                    instructions.extend(translator(translated_args));
                }
                _ => {
                    unimplemented!("Function is not supported");
                }
            }
            BoatArg::FromStack
        },
        BoatExpr::BinOp { lhs, op, rhs } => {
            let mut instructions = Vec::<BoatIns>::new();
            let mut bin_op_ins = BoatIns { cmd: op.into(), args: vec![] };
            bin_op_ins.args.push(translate_expr(*rhs, &mut instructions, functions));
            bin_op_ins.args.push(translate_expr(*lhs, &mut instructions, functions));
            instructions.push(bin_op_ins);
            BoatArg::FromStack
        },
    }
}