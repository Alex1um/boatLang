mod expr_parser;
mod expr_optimizer;
mod expr_translator;
mod program_parser;
mod boat_instructions;
mod boat_program;
mod program_translator;
mod interpreter;





#[cfg(not(debug_assertions))]
fn main() {
    use std::fs;
    use std::path::PathBuf;

    let flags = xflags::parse_or_exit! {
        /// Interpret compiled code
        optional -i,--interpret
        /// Print prettified compiled code
        optional -p,--preety
        /// Use programs v2
        optional -v2,--version2
        /// Use debug mode in interpreter
        optional -d,--debug
        /// File or directory to parse
        required path: PathBuf
    };
    let contents = fs::read_to_string(flags.path.to_str().expect("Unable to read the file")).expect("Unable to read the file");
    let program = program_parser::parse_program(&contents);
    let translated = crate::program_translator::translate_program(program);
    if flags.preety {
        crate::boat_instructions::translated_debug(&translated);
    }
    if flags.interpret {
        crate::interpreter::interpret(&translated, flags.debug);
    }
    let text = if flags.version2 {
        crate::boat_instructions::translated_to_string2(translated)
    } else {
        crate::boat_instructions::translated_to_string(translated)
    };
    println!("{}", text);
}

#[cfg(debug_assertions)]
fn main() {
    
    let contents = "
    input = in(1);
    print = out(1);
    {
        max = input();
        i = 1;
        sum = 1;
        while (max > i) {
            sum = sum * i;
            i = i + 1;
        }
        print(\"result:\");
        print(sum);
    }
    ";

    // Open the file, read it, and parse it
    let program = program_parser::parse_program(contents);
    let translated = crate::program_translator::translate_program(program);
    crate::boat_instructions::translated_debug(&translated);
    crate::interpreter::interpret(&translated, false);
    let text = crate::boat_instructions::translated_to_string(translated);
    println!("{}", text);
}