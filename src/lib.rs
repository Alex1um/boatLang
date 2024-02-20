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
        // if flags.preety {
        //     crate::boat_instructions::translated_debug(&translated);
        // }
        // if flags.interpret {
        //     crate::interpreter::interpret(&translated, flags.debug);
        // }
        // let text = if flags.version2 {
            crate::boat_instructions::translated_to_string2(translated)
        // } else {
        //     crate::boat_instructions::translated_to_string(translated)
        // };
        // text
    }
}
