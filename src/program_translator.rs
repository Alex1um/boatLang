use crate::boat_instructions::BoatIns;
use crate::expr_translator::translate_expr;
use crate::boat_program::{Statement, Functions, Block, Program};

// current_ins_i = index of last instruction + 1
fn translate_statement(s: Statement, mut instruction_index: u32, functions: &Functions) -> Vec<BoatIns> {
    match s {
        Statement::Assign { var_name, expr } => {
            let mut instructions = translate_expr(expr, functions);
            instructions.push(BoatIns::KVDel { key: var_name.clone() });
            instructions.push(BoatIns::KVSet { key: var_name });
            instructions
        }
        Statement::If { expr, block, else_block } => {
            let mut statement = Vec::<BoatIns>::new();
            let mut expr = translate_expr(expr, functions);
            instruction_index += expr.len() as u32 + 3;
            let mut block = translate_block(block, instruction_index, functions);
            instruction_index += block.len() as u32;
            statement.append(&mut expr);
            statement.push(BoatIns::Push { value: "0".to_owned() });
            statement.push(BoatIns::Eq);
            if !else_block.is_none() {
                instruction_index += 1;
            }
            statement.push(BoatIns::Cmp { ins: instruction_index });
            statement.append(&mut block);
            if let Some(else_block) = else_block {
                let mut else_block = translate_block(else_block, instruction_index, functions);
                statement.push(BoatIns::Goto { ins: instruction_index + else_block.len() as u32 });
                statement.append(&mut else_block);
            }
            statement
        }
        Statement::While { expr, block } => {
            let mut statement = Vec::<BoatIns>::new();
            let mut expr = translate_expr(expr, functions);
            let while_begin_index = instruction_index;
            instruction_index += expr.len() as u32 + 3;
            let mut block = translate_block(block, instruction_index, functions);
            instruction_index += block.len() as u32 + 1;
            statement.extend( expr);
            statement.push(BoatIns::Push { value: "0".to_owned() });
            statement.push(BoatIns::Eq);
            statement.push(BoatIns::Cmp { ins: instruction_index });
            statement.extend(block);
            statement.push(BoatIns::Goto { ins: while_begin_index });
            statement
        }
        Statement::Expr(expr) => {
            let mut instructions = translate_expr(expr, functions);
            // instructions.push(BoatIns::Pop);
            instructions
        }
        _ => unimplemented!("Unsupported statement: {:?}", s),
    }
}

pub fn translate_block(block: Block, mut instruction_index: u32, functions: &Functions) -> Vec<BoatIns> {
    block.into_iter().map(|statement| {
        let translated = translate_statement(statement, instruction_index, functions);
        instruction_index += translated.len() as u32;
        translated
    }).flatten().collect()
}

pub fn translate_program(program: Program) -> Vec<BoatIns> {
    let Program { functions, block } = program;
    translate_block(block, 1, &functions)
}
