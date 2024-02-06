use std::collections::HashMap;

mod expr_parser;
mod expr_optimizer;
mod expr_translator;
mod program_parser;
mod boat_instructions;
mod boat_program;
mod program_translator;

fn main() {
    let program = program_parser::parse_program("
    print=out(1);
    input=in(1);
    {
        a = 1 + 1;
        while (10 - a > 0) {
            a = a + 1;
        }
        if (a == 10) {
            a = 0;
        }
        if (a == 0) {
            b = 1;
        } else {
            b = 2;
        }
    }
    ");
    let translated = crate::program_translator::translate_program(program);
    println!("{:#?}", translated);
    let text = crate::boat_instructions::translated_to_string(translated);
    println!("{}", text);
}