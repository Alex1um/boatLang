use crate::boat_program::{BoatExpr, BoatOp, Function, Functions};
use crate::boat_instructions::{BoatIns, BoatArg, BoatCmd};

impl From<BoatOp> for BoatCmd {
    fn from(val: BoatOp) -> Self {
        use BoatOp::*;
        match val {
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

pub fn translate_expr(arg: BoatExpr, instruction_index: &mut u32, instructions: &mut Vec<BoatIns>, functions: &Functions) -> BoatArg {
    match arg {
        BoatExpr::Value(value) => BoatArg::Const(value),
        BoatExpr::Var(name) => BoatArg::FromKVS(name),
        BoatExpr::Function { name, mut args } => {
            let function = functions.get(&name).expect("Function is defined");
            let mut translated_args = Vec::<BoatArg>::new();
            args.reverse();
            for arg in args {
                translated_args.push(translate_expr(arg, instruction_index, instructions, functions))
            }
            translated_args.reverse();
            match function {
                Function::Predefined { translator } => {
                    let translated_instrutions = translator(translated_args);
                    *instruction_index += translated_instrutions.len() as u32;
                    instructions.extend(translated_instrutions);
                }
                Function::InProgram { begin_pos, arg_names } => {
                    for (arg, name) in translated_args.into_iter().zip(arg_names) {
                        *instruction_index += 1;
                        instructions.push(BoatIns { cmd: BoatCmd::KVSet, args: vec![BoatArg::Const(name.to_string()), arg] });
                    }
                    *instruction_index += 2;
                    instructions.push(BoatIns { cmd: BoatCmd::KVSet, args: vec![BoatArg::Const("return".to_owned()), BoatArg::Const(instruction_index.to_string())] });
                    instructions.push(BoatIns { cmd: BoatCmd::Goto, args: vec![BoatArg::Const(begin_pos.to_string())] });
                    instructions.push(BoatIns { cmd: BoatCmd::KVDel, args: vec![BoatArg::Const("return".to_owned())] });
                    *instruction_index += 1;
                    for name in arg_names {
                        *instruction_index += 1;
                        instructions.push(BoatIns { cmd: BoatCmd::KVDel, args: vec![BoatArg::Const(name.to_string())] });
                    }
                }
            }
            BoatArg::FromStack
        },
        BoatExpr::BinOp { lhs, op, rhs } => {
            let mut bin_op_ins = BoatIns { cmd: op.into(), args: vec![] };
            let rhs_arg = translate_expr(*rhs, instruction_index, instructions, functions);
            let lhs_arg = translate_expr(*lhs, instruction_index, instructions, functions);
            if rhs_arg == BoatArg::FromStack && lhs_arg == BoatArg::FromStack {
                bin_op_ins.args.push(rhs_arg);
                bin_op_ins.args.push(lhs_arg);
            } else {
                bin_op_ins.args.push(lhs_arg);
                bin_op_ins.args.push(rhs_arg);
            }
            instructions.push(bin_op_ins);
            *instruction_index += 1;
            BoatArg::FromStack
        },
    }
}