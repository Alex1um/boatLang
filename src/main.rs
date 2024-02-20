use crate::program_optimizer::optimize_reassigns;

mod expr_parser;
mod expr_optimizer;
mod expr_translator;
mod program_parser;
mod boat_instructions;
mod boat_program;
mod program_translator;
mod interpreter;
mod program_optimizer;




fn main() {
    use std::fs;
    use std::path::PathBuf;

    let flags = xflags::parse_or_exit! {
        /// Interpret compiled code
        optional -i,--interpret
        /// Print prettified compiled code
        optional -p,--preety
        /// Use old program type
        optional -l,--legacy
        /// Use debug mode in interpreter
        optional -d,--debug
        /// File or directory to parse
        required path: PathBuf
    };
    let contents = fs::read_to_string(flags.path.to_str().expect("Unable to read the file")).expect("Unable to read the file");
    let mut program = match program_parser::parse_program(&contents) {
        Ok(program) => program,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };
    optimize_reassigns(&mut program);
    let translated = crate::program_translator::translate_program(program);
    if flags.preety {
        crate::boat_instructions::translated_debug(&translated);
    }
    if flags.interpret {
        crate::interpreter::interpret(&translated, flags.debug);
    }
    let text = if flags.legacy {
        crate::boat_instructions::translated_to_string(translated)
    } else {
        crate::boat_instructions::translated_to_string2(translated)
    };
    println!("{}", text);
}

