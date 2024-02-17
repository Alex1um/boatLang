use std::collections::HashSet;

use crate::boat_program::{Block, Program, Statement};

fn optimize_block_reassigns(block: &mut Block, current_vars: &mut HashSet<String>) {
    for s in block {
        match s {
            Statement::Assign { ref var_name, ref expr } => {
                if current_vars.contains(var_name.as_str()) {
                    // let _ = std::mem::replace(s, Statement::Assign { var_name: var_name.clone(), expr: expr.clone() });
                    *s = Statement::Reassign { var_name: var_name.clone(), expr: expr.clone() };
                } else {
                    current_vars.insert(var_name.clone());
                }
            }
            Statement::If { expr, block, else_block } => {
                optimize_block_reassigns(block, current_vars);
                if let Some(else_block) = else_block {
                    optimize_block_reassigns(else_block, current_vars);
                }
            }
            Statement::While { expr, block } => {
                optimize_block_reassigns(block, current_vars);
            }
            Statement::FunctionDefinition { name, arg_names, block } => {
                for name in arg_names.iter() {
                    current_vars.insert(name.clone());
                }
                optimize_block_reassigns(block, current_vars);
                for name in arg_names.iter() {
                    current_vars.remove(name.as_str());
                }
            }
            _ => {}
        }
    }
}

pub fn optimize_reassigns(program: &mut Program) {
    let mut vars = HashSet::<String>::new();
    optimize_block_reassigns(&mut program.block, &mut vars);
}