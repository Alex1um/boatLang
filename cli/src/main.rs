extern crate boat_lang_core;

use crate::boat_lang_core::{
    program_parser,
    program_translator,
    program_optimizer,
    interpreter,
    boat_instructions,
};

use std::io;


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
    program_optimizer::optimize_reassigns(&mut program);
    let translated = program_translator::translate_program(program);
    if flags.interpret {
        let out = io::stdout();
        let inp = io::stdin().lock();
        interpreter::interpret(&translated, out, inp, flags.debug);
    }
    let text = if flags.legacy {
        boat_instructions::translated_to_string(translated)
    } else {
        boat_instructions::translated_to_string2(translated, flags.preety)
    };
    println!("{}", text);
}

