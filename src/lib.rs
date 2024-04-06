mod expr_parser;
mod expr_optimizer;
mod expr_translator;
mod program_parser;
mod boat_instructions;
mod boat_program;
mod program_translator;
mod interpreter;
mod program_optimizer;



#[cfg(target_family="wasm")]
mod wasm {
    use wasm_bindgen::prelude::*;
    use super::*;

    #[wasm_bindgen]
    pub fn boat_lang_compile(contents: String) -> String {
        let mut program = match program_parser::parse_program(&contents) {
            Ok(program) => program,
            Err(e) => {
                // println!("{}", e);
                return format!("{e}");
            }
        };
        program_optimizer:: optimize_reassigns(&mut program);
        let translated = crate::program_translator::translate_program(program);
        crate::boat_instructions::translated_to_string2(translated)
    }

    #[wasm_bindgen]
    pub fn boat_lang_interpret(contents: String, debug: bool) -> String {
        let mut program = match program_parser::parse_program(&contents) {
            Ok(program) => program,
            Err(e) => {
                // println!("{}", e);
                return format!("{e}");
            }
        };
        program_optimizer:: optimize_reassigns(&mut program);
        let translated = crate::program_translator::translate_program(program);
        let mut out = Vec::<u8>::new();
        crate::interpreter::interpret(&translated, &mut out, flags.debug);
        String::from_utf8(out).expect("output is valid utf8")
    }
}
