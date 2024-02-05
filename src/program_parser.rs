use pest::{iterators::Pairs, Parser};
use crate::expr_parser::BoatExpr;

type Block = Vec<Statement>;

#[derive(Debug)]
pub enum Statement {
    If { expr: BoatExpr, block: Block, else_block: Option<Block> },
    While { expr: BoatExpr, block: Block },
    Assign { var_name: String, expr: BoatExpr },
}

#[derive(Debug)]
pub struct FunctionDefinition {
    pub name: String,
    pub args: Vec<String>,
    pub place: u32,
}

#[derive(Debug)]
pub enum PinType {
    In(u32),
    Out(u32),
}

#[derive(Debug)]
pub struct PinDefinition {
    pub name: String,
    pub pin: PinType,
}

#[derive(pest_derive::Parser)]
#[grammar = "program.pest"]
#[grammar = "expr.pest"]
#[grammar = "base.pest"]
pub struct ProgramParser;

#[derive(Debug)]
pub struct Program {
    pub pin_definitions: Vec<PinDefinition>,
    pub functions: Vec<FunctionDefinition>,
    pub block: Block,
}

pub fn parse_definitions(pairs: Pairs<Rule>) -> Vec<PinDefinition> {
    let mut definitions = Vec::new();
    for pair in pairs {
        println!("{:#?}", pair);
    }
    definitions
}

pub fn parse_program(s: &str) {
    let parsed = ProgramParser::parse(Rule::definition_section, s);
    println!("{:#?}", parsed);

}
