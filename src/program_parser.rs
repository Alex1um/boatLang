use std::collections::HashMap;

use pest::{iterators::Pairs, Parser};
use crate::{boat_instructions::BoatIns, expr_parser::parse_pairs, boat_program::{Program, Block, Statement, Function}};



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

pub fn parse_block(pairs: Pairs<Rule>) -> Block {
    pairs.into_iter().map(|pair| {
        match pair.as_rule() {
            Rule::r#if => {
                let mut inner = pair.into_inner();
                Statement::If {
                    expr: parse_pairs(inner.next().unwrap().into_inner()),
                    block: parse_block(inner.next().unwrap().into_inner()),
                    else_block: inner.next().map(|pair| parse_block(pair.into_inner()))
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
            Rule::expr => {
                Statement::Expr(parse_pairs(pair.into_inner()))
            }
            _ => unreachable!()
        }
    }).collect()
}

pub fn parse_program(s: &str) -> Program {
    let mut functions = HashMap::<String, Function>::new();
    let mut parsed = ProgramParser::parse(Rule::program, s).unwrap();
    let mut program = parsed.next().unwrap().into_inner();
    let definitions_pairs = program.next().unwrap().into_inner();
    let main_block_pairs = program.next().unwrap().into_inner();
    let pin_definitions = parse_definitions(definitions_pairs);
    for pin_def in pin_definitions {
        functions.insert(pin_def.name, Function::Predefined { instructions: vec![
            match pin_def.pin {
                PinType::In(in_num) => BoatIns::Input { pin: in_num },
                PinType::Out(out_num) => BoatIns::Output { pin: out_num }
            }
        ] });
    }
    let block = parse_block(main_block_pairs);
    Program { functions: functions, block: block }
}
