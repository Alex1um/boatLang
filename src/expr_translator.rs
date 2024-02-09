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
            let function = functions.get(&name).expect("Function is defined");
            let mut translated_args = Vec::<BoatArg>::new();
            args.reverse();
            for arg in args {
                translated_args.push(translate_expr(arg, instructions, functions))
            }
            translated_args.reverse();
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
            let mut bin_op_ins = BoatIns { cmd: op.into(), args: vec![] };
            let rhs_arg = translate_expr(*rhs, instructions, functions);
            let lhs_arg = translate_expr(*lhs, instructions, functions);
            if rhs_arg == BoatArg::FromStack && lhs_arg == BoatArg::FromStack {
                bin_op_ins.args.push(rhs_arg);
                bin_op_ins.args.push(lhs_arg);
            } else {
                bin_op_ins.args.push(lhs_arg);
                bin_op_ins.args.push(rhs_arg);
            }
            instructions.push(bin_op_ins);
            BoatArg::FromStack
        },
    }
}