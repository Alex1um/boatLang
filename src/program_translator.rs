use crate::boat_instructions::{BoatIns, BoatArg, BoatCmd};
use crate::expr_translator::translate_expr;
use crate::boat_program::{Statement, Functions, Block, Program, Function};

// current_ins_i = index of last instruction + 1
fn translate_statement(s: Statement, mut instruction_index: u32, functions: &mut Functions) -> Vec<BoatIns> {
    match s {
        Statement::Assign { var_name, expr } => {
            let mut instructions = Vec::<BoatIns>::new();
            let arg = translate_expr(expr, &mut instructions, functions);
            instructions.push(BoatIns { cmd: BoatCmd::KVSet, args: vec![BoatArg::Const(var_name), arg] });
            instructions
        }
        Statement::If { expr, block, else_block } => {
            let mut statement = Vec::<BoatIns>::new();
            let while_arg = translate_expr(expr, &mut statement, functions);
            instruction_index += statement.len() as u32 + 2; // cmd and eq
            let block = translate_block(block, instruction_index, functions);
            instruction_index += block.len() as u32;
            
            statement.push(BoatIns {cmd: BoatCmd::Eq, args: vec![while_arg, BoatArg::Const("0".to_owned())]});
            if !else_block.is_none() {
                instruction_index += 1;
            }
            statement.push(BoatIns {cmd: BoatCmd::Cmp, args: vec![BoatArg::FromStack, BoatArg::Const(instruction_index.to_string())]});
            statement.extend(block);

            if let Some(else_block) = else_block {
                let mut else_block = translate_block(else_block, instruction_index, functions);
                instruction_index += else_block.len() as u32;
                statement.push(BoatIns { cmd: BoatCmd::Goto, args: vec![ BoatArg::Const(instruction_index.to_string()) ] });
                statement.extend(else_block);
            }
            statement
        }
        Statement::While { expr, block } => {
            let mut statement = Vec::<BoatIns>::new();
            let while_arg = translate_expr(expr, &mut statement, functions);
            let while_begin_index = instruction_index;
            instruction_index += statement.len() as u32 + 2;
            let block = translate_block(block, instruction_index, functions);
            instruction_index += block.len() as u32 + 1;
            statement.push(BoatIns { cmd: BoatCmd::Eq, args: vec![while_arg, BoatArg::Const("0".to_owned())]});
            statement.push(BoatIns { cmd: BoatCmd::Cmp, args: vec![BoatArg::FromStack, BoatArg::Const(instruction_index.to_string())] });
            statement.extend(block);
            statement.push(BoatIns { cmd: BoatCmd::Goto, args: vec![ BoatArg::Const(while_begin_index.to_string()) ] });
            statement
        }
        Statement::Expr(expr) => {
            let mut instructions = Vec::<BoatIns>::new();
            let mut arg = translate_expr(expr, &mut instructions, functions);
            instructions
        }
        Statement::FunctionDefinition { name, arg_names, block } => {
            let mut instructions = Vec::<BoatIns>::new();
            
            functions.insert(name, Function::InProgram { begin_pos: instruction_index });

            instruction_index += arg_names.len() as u32;
            for arg_name in arg_names {
                instructions.push(BoatIns { cmd: BoatCmd::KVSet, args: vec![BoatArg::Const(arg_name), BoatArg::FromStack] });
            }
            instructions.extend(translate_block(block, instruction_index, functions));
            instructions
        }
        _ => unimplemented!("Unsupported statement: {:?}", s),
    }
}

pub fn translate_block(block: Block, mut instruction_index: u32, functions: &mut Functions) -> Vec<BoatIns> {
    block.into_iter().map(|statement| {
        let translated = translate_statement(statement, instruction_index, functions);
        instruction_index += translated.len() as u32;
        translated
    }).flatten().collect()
}

pub fn translate_program(program: Program) -> Vec<BoatIns> {
    let Program { mut functions, block } = program;
    translate_block(block, 1, &mut functions)
}
