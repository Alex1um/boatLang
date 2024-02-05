use std::collections::HashMap;

mod expr_parser;
mod expr_optimizer;
mod expr_translator;
mod program_parser;

fn main() {
    // let parsed = expr_parser::parse_string("2 + 3 * 4 / 9");
    // println!("{:?}", parsed);
    // let optimized = expr_optimizer::optimize_expr(parsed);
    // println!("{:?}", optimized);
    // let translated = expr_translator::translate_expr(optimized, &HashMap::new());
    // println!("{:?}", translated);
    // println!("{}", expr_translator::translated_to_string(translated));
    program_parser::parse_program("print=out(1)")
}