use std::collections::HashMap;

use pest::{iterators::Pairs, Parser};
use crate::{boat_instructions::{BoatArg, BoatCmd, BoatIns}, boat_program::{Block, Function, Program, Statement}, expr_parser::parse_pairs};



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
            Rule::function_definition => {
                let mut inner = pair.into_inner();
                let name = inner.next().unwrap().as_str().to_owned();
                let mut arg = inner.next().unwrap();
                let mut args = Vec::<String>::new();
                while arg.as_rule() == Rule::name {
                    args.push(arg.as_str().to_owned());
                    arg = inner.next().unwrap();
                }
                Statement::FunctionDefinition { name, arg_names: args,  block: parse_block(arg.into_inner()) }
            }
            _ => unreachable!()
        }
    }).collect()
}

pub fn parse_program(s: &str) -> Result<Program, pest::error::Error<Rule>> {
    let mut functions = HashMap::<String, Function>::new();
    let mut parsed = ProgramParser::parse(Rule::program, s)?;
    let mut program = parsed.next().unwrap().into_inner();
    let definitions_pairs = program.next().unwrap().into_inner();
    let main_block_pairs = program.next().unwrap().into_inner();
    let pin_definitions = parse_definitions(definitions_pairs);
    for pin_def in pin_definitions {
        let pin_num = pin_def.pin;
        functions.insert(pin_def.name, Function::Predefined { translator: Box::new(move |mut args: Vec<BoatArg>| {
            let (tpe, num) = match pin_num {
                PinType::In(i) => (BoatCmd::Input, i),
                PinType::Out(i) => (BoatCmd::Output, i),
            };
            args.insert(0, BoatArg::Const(num.to_string()));
            if args.len() == 1 && tpe == BoatCmd::Input {
                args.insert(1, BoatArg::Const("60".to_owned()));
            }
            vec![ BoatIns { cmd: tpe, args } ]
        }) });
    }
    functions.insert("sleep".to_owned(), Function::Predefined { translator: Box::new(|args: Vec<BoatArg>| {
        vec![ BoatIns { cmd: BoatCmd::Sleep, args } ]
    }) });
    functions.insert("display".to_owned(), Function::Predefined { translator: Box::new(|args: Vec<BoatArg>| {
        vec![ BoatIns { cmd: BoatCmd::Display, args } ]
    }) });
    functions.insert("dclear".to_owned(), Function::Predefined { translator: Box::new(|args: Vec<BoatArg>| {
        vec![ BoatIns { cmd: BoatCmd::DisplayClear, args } ]
    }) });
    functions.insert("out".to_owned(), Function::Predefined { translator: Box::new(|args: Vec<BoatArg>| {
        vec![ BoatIns { cmd: BoatCmd::Output, args } ]
    }) });
    functions.insert("in".to_owned(), Function::Predefined { translator: Box::new(|args: Vec<BoatArg>| {
        vec![ BoatIns { cmd: BoatCmd::Input, args } ]
    }) });
    functions.insert("clear".to_owned(), Function::Predefined { translator: Box::new(|args: Vec<BoatArg>| {
        vec![ BoatIns { cmd: BoatCmd::Clear, args } ]
    }) });
    functions.insert("store".to_owned(), Function::Predefined { translator: Box::new(|args: Vec<BoatArg>| {
        vec![ BoatIns { cmd: BoatCmd::Store, args } ]
    }) });
    let block = parse_block(main_block_pairs);
    Ok(Program { functions, block })
}
