use pest::Parser;

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
}

pub fn parse_program(s: &str) {
    let parsed = ProgramParser::parse(Rule::definition_section, s);
    println!("{:#?}", parsed);
}
