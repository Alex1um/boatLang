
use std::collections::HashMap;

use crate::boat_instructions::BoatIns;
use crate::expr_translator::translate_expr;
use crate::boat_program::{Statement, Functions, Block, Program};

// current_ins_i = index of last instruction + 1
fn translate_statement(s: Statement, current_ins_i: u32, functions: &Functions) -> Vec<BoatIns> {
    match s {
        Statement::Assign { var_name, expr } => {
            let mut expr = translate_expr(expr, functions);
            expr.push(BoatIns::KVSet { key: var_name });
            expr
        }
        Statement::If { expr, block, else_block } => {
            let mut statement = Vec::<BoatIns>::new();
            let mut expr = translate_expr(expr, functions);
            let mut block = translate_block(block, functions);
            statement.append(&mut expr);
            statement.push(BoatIns::Push { value: "0".to_owned() });
            statement.push(BoatIns::Eq);
            statement.push(BoatIns::Cmp { ins: current_ins_i + expr.len() as u32 + block.len() as u32 + 2 });
            statement.append(&mut block);
            if let Some(else_block) = else_block {
                let mut else_block = translate_block(else_block, functions);
                statement.push(BoatIns::Goto { ins: current_ins_i + expr.len() as u32 + block.len() as u32 + else_block.len() as u32 + 2 });
                statement.append(&mut else_block);
            }
            statement
        }
        Statement::While { expr, block } => {
            let mut statement = Vec::<BoatIns>::new();
            let mut expr = translate_expr(expr, functions);
            let mut block = translate_block(block, functions);
            statement.append(&mut expr);
            statement.push(BoatIns::Push { value: "0".to_owned() });
            statement.push(BoatIns::Eq);
            statement.push(BoatIns::Cmp { ins: current_ins_i + expr.len() as u32 + block.len() as u32 + 3 });
            statement.append(&mut block);
            statement.push(BoatIns::Goto { ins: current_ins_i });
            statement
        }
        _ => unimplemented!("Unsupported statement: {:?}", s),
    }
}

pub fn translate_block(block: Block, functions: &Functions) -> Vec<BoatIns> {
    let mut instruction_index = 1u32;
    block.into_iter().map(|statement| {
        let translated = translate_statement(statement, instruction_index, functions);
        instruction_index += translated.len() as u32;
        translated
    }).flatten().collect()
}

pub fn translate_program(program: Program) -> Vec<BoatIns> {
    let Program { functions, block } = program;
    translate_block(block, &functions)
}
