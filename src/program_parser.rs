use pest::{iterators::Pairs, Parser};
use crate::expr_parser::{BoatExpr, parse_pairs};

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
    pairs.into_iter().map(|pair| {
        let mut inner = pair.into_inner();
        let name = inner.next().unwrap().as_str().to_owned();
        let f = inner.next().unwrap();
        let tpe = match f.as_rule() {
            Rule::in_f => PinType::In,
            Rule::out_f => PinType::Out,
            _ => unreachable!()
        };
        let inner = f.into_inner().next().unwrap();
        let pin = inner.as_str().parse::<u32>().unwrap();
        PinDefinition { name, pin: tpe(pin) }
    }).collect()
}

pub fn parse_block(pairs: Pairs<Rule>) -> Vec<Statement> {
    pairs.into_iter().map(|pair| {
        match pair.as_rule() {
            Rule::r#if => {
                let mut inner = pair.into_inner();
                Statement::If {
                    expr: parse_pairs(inner.next().unwrap().into_inner()),
                    block: parse_block(inner.next().unwrap().into_inner()),
                    else_block: None
                }
            },
            Rule::r#while => {
                let mut inner = pair.into_inner();
                Statement::While {
                    expr: parse_pairs(inner.next().unwrap().into_inner()),
                    block: parse_block(inner.next().unwrap().into_inner()),
                }
            },
            Rule::assign => {
                let mut inner = pair.into_inner();
                Statement::Assign {
                    var_name: inner.next().unwrap().as_str().to_owned(),
                    expr: parse_pairs(inner.next().unwrap().into_inner()),
                }
            },
            _ => unreachable!()
        }
    }).collect()
}

pub fn parse_program(s: &str) {
    let mut parsed = ProgramParser::parse(Rule::program, s).unwrap();
    let mut program = parsed.next().unwrap().into_inner();
    let definitions_pairs = program.next().unwrap().into_inner();
    let main_block_pairs = program.next().unwrap().into_inner();
    // println!("{:#?}", definitions_pairs);
    // println!("{:#?}", main_block_pairs);
    let pin_definitions = parse_definitions(definitions_pairs);
    println!("{:#?}", pin_definitions);
    let block = parse_block(main_block_pairs);
    println!("{:#?}", block);
}
