use std::collections::HashSet;

use crate::boat_instructions::{BoatIns, BoatArg, BoatCmd};
use crate::expr_translator::translate_expr;
use crate::boat_program::{Block, BoatExpr, Function, Functions, Program, Statement};

// current_ins_i = index of last instruction + 1
fn translate_statement(s: Statement, instruction_index: &mut u32, functions: &mut Functions, labeled_lines: &mut HashSet<u32>) -> Vec<BoatIns> {
    match s {
        Statement::Reassign { var_name, expr } => {
            let mut instructions = Vec::<BoatIns>::new();
            let arg = translate_expr(expr, instruction_index, &mut instructions, functions, labeled_lines);
            instructions.push(BoatIns { cmd: BoatCmd::KVReSet, args: vec![BoatArg::Const(var_name), arg] });
            *instruction_index += 1;
            instructions
        }
        Statement::Assign { var_name, expr } => {
            let mut instructions = Vec::<BoatIns>::new();
            let arg = translate_expr(expr, instruction_index, &mut instructions, functions, labeled_lines);
            // instructions.push(BoatIns { cmd: BoatCmd::KVDel, args: vec![BoatArg::Const(var_name.clone())] });
            // *instruction_index += 1;
            instructions.push(BoatIns { cmd: BoatCmd::KVSet, args: vec![BoatArg::Const(var_name), arg] });
            *instruction_index += 1;
            instructions
        }
        Statement::If { expr, block, else_block } => {
            let mut statement = Vec::<BoatIns>::new();
            let if_arg = translate_expr(expr, instruction_index, &mut statement, functions, labeled_lines);
            *instruction_index += 1; // cmd and eq
            let block = translate_block(block, instruction_index, functions, labeled_lines);
            // *instruction_index += block.len() as u32;
            
            if else_block.is_some() {
                *instruction_index += 1;
            }
            statement.push(BoatIns {cmd: BoatCmd::Cmp, args: vec![if_arg, BoatArg::Const(instruction_index.to_string())]});
            statement.extend(block);

            if let Some(else_block) = else_block {
                let else_block = translate_block(else_block, instruction_index, functions, labeled_lines);
                statement.push(BoatIns { cmd: BoatCmd::Goto, args: vec![ BoatArg::Const(instruction_index.to_string()) ] });
                labeled_lines.insert(*instruction_index);
                // *instruction_index += else_block.len() as u32;
                statement.extend(else_block);
            }
            statement
        }
        Statement::While { expr, block } => {
            let mut statement = Vec::<BoatIns>::new();
            let while_begin_index = *instruction_index;
            let while_arg = translate_expr(expr, instruction_index, &mut statement, functions, labeled_lines);
            *instruction_index += 1;
            let block = translate_block(block, instruction_index, functions, labeled_lines);
            statement.push(BoatIns { cmd: BoatCmd::Cmp, args: vec![while_arg, BoatArg::Const((*instruction_index + 1u32).to_string())] });
            statement.extend(block);
            statement.push(BoatIns { cmd: BoatCmd::Goto, args: vec![ BoatArg::Const(while_begin_index.to_string()) ] });
            labeled_lines.insert(while_begin_index);
            *instruction_index += 1;
            statement
        }
        Statement::Return(expr) => {
            let mut instructions = Vec::<BoatIns>::new();
            // let is_push_needed = matches!(expr, BoatExpr::Value(_) | BoatExpr::Var(_));
            let arg = translate_expr(expr, instruction_index, &mut instructions, functions, labeled_lines);
            if arg != BoatArg::FromStack {
                *instruction_index += 1;
                instructions.push(BoatIns { cmd: BoatCmd::Push, args: vec![arg] });
            }
            instructions
        }
        Statement::Expr(expr) => {
            let mut instructions = Vec::<BoatIns>::new();
            let _ = translate_expr(expr, instruction_index, &mut instructions, functions, labeled_lines);
            instructions
        }
        Statement::FunctionDefinition { name, arg_names, block } => {
            let mut instructions = Vec::<BoatIns>::new();
            
            *instruction_index += 1;
            functions.insert(name, Function::InProgram { begin_pos: *instruction_index, arg_names });
            labeled_lines.insert(*instruction_index);

            instructions.extend(translate_block(block, instruction_index, functions, labeled_lines));
            instructions.push(BoatIns { cmd: BoatCmd::Goto, args: vec![BoatArg::FromKVS("return".to_string())] });
            *instruction_index += 1;
            instructions.insert(0, BoatIns { cmd: BoatCmd::Goto, args: vec![BoatArg::Const(instruction_index.to_string())] });
            labeled_lines.insert(*instruction_index);
            instructions
        }
    }
}

pub fn translate_block(block: Block, instruction_index: &mut u32, functions: &mut Functions, labeled_lines: &mut HashSet<u32>) -> Vec<BoatIns> {
    block.into_iter().flat_map(|statement| {
        
        // instruction_index += translated.len() as u32;
        translate_statement(statement, instruction_index, functions, labeled_lines)
    }).collect()
}

pub fn translate_program(program: Program, labeled_lines: &mut HashSet<u32>) -> Vec<BoatIns> {
    let Program { mut functions, block } = program;
    translate_block(block, &mut 1, &mut functions, labeled_lines)
}
