use crate::boat_instructions::{BoatArg, BoatIns};
use std::collections::HashMap;

#[derive(Debug)]
pub enum BoatExpr {
    Value(String),
    Var(String),
    Function {
        name: String,
        args: Vec<BoatExpr>,
    },
    BinOp {
        lhs: Box<BoatExpr>,
        op: BoatOp,
        rhs: Box<BoatExpr>,
    },
}

#[derive(Debug)]
pub enum BoatOp {
    Add,
    Sub,
    Mul,
    Div,
    Conc,
    Gt,
    Eq,
}

#[derive(Debug)]
pub enum Statement {
    If { expr: BoatExpr, block: Block, else_block: Option<Block> },
    While { expr: BoatExpr, block: Block },
    Assign { var_name: String, expr: BoatExpr },
    Expr(BoatExpr),
}

pub type Block = Vec<Statement>;

pub enum Function {
    KeyFunction {
        key: String,
        arg_names: Vec<String>
    },
    InProgram {
        begin_pos: u32,
        arg_names: Vec<String>
    },
    Predefined {
        translator: Box<dyn Fn(Vec<BoatArg>) -> Vec<BoatIns>>
    }
}

pub type Functions = HashMap<String, Function>;

pub struct Program {
    pub functions: Functions,
    pub block: Block,
}