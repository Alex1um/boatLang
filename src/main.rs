use std::collections::HashMap;

mod expr_parser;
mod expr_optimizer;
mod expr_translator;
mod program_parser;
mod boat_instructions;

fn main() {
    program_parser::parse_program("
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
}